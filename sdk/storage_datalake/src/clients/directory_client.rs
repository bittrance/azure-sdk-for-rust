use crate::operations::*;
use crate::request_options::*;
use crate::{
    clients::{DataLakeClient, FileSystemClient},
    prelude::PathClient,
    Properties,
};
use azure_core::prelude::IfMatchCondition;
use azure_core::{ClientOptions, Context, Pipeline};
use azure_storage::core::storage_shared_key_credential::StorageSharedKeyCredential;
use url::Url;

#[derive(Debug, Clone)]
pub struct DirectoryClient {
    file_system_client: FileSystemClient,
    dir_path: String,
}

impl PathClient for DirectoryClient {
    fn url(&self) -> azure_core::Result<Url> {
        let fs_url = self.file_system_client.url()?;
        let dir_path = vec![fs_url.path(), &self.dir_path].join("/");
        Ok(self.file_system_client.url()?.join(&dir_path)?)
    }

    fn pipeline(&self) -> &Pipeline {
        self.file_system_client.pipeline()
    }

    fn context(&self) -> &Context {
        &self.file_system_client.context
    }
}

impl DirectoryClient {
    pub(crate) fn new(file_system_client: FileSystemClient, path: String) -> Self {
        Self {
            file_system_client,
            dir_path: path,
        }
    }

    pub fn new_with_options<FS, D>(
        credential: StorageSharedKeyCredential,
        custom_dns_suffix: Option<String>,
        options: ClientOptions,
        file_system_name: FS,
        path: D,
    ) -> Self
    where
        FS: Into<String>,
        D: Into<String>,
    {
        DataLakeClient::new_with_shared_key(credential, custom_dns_suffix, options)
            .into_file_system_client(file_system_name.into())
            .into_directory_client(path)
    }

    pub fn list_paths(&self) -> ListPathsBuilder {
        let fs_url = self.file_system_client.url().unwrap();
        // the path will contain a leading '/' as we extract if from the path component of the url
        let dir_path = vec![fs_url.path(), &self.dir_path].join("/");
        ListPathsBuilder::new(self.file_system_client.clone(), self.context().clone())
            .directory(dir_path)
            .recursive(true)
    }

    pub fn create(&self) -> PutPathBuilder<Self> {
        PutPathBuilder::new(self.clone(), self.file_system_client.context.clone())
            .resource(ResourceType::Directory)
    }

    pub fn create_if_not_exists(&self) -> PutPathBuilder<Self> {
        self.create()
            .if_match_condition(IfMatchCondition::NotMatch("*".to_string()))
    }

    pub fn rename<P>(&self, destination_path: P) -> RenamePathBuilder<Self>
    where
        P: Into<String>,
    {
        let destination_client = self
            .file_system_client
            .get_directory_client(destination_path);
        let fs_url = self.file_system_client.url().unwrap();
        // the path will contain a leading '/' as we extract if from the path component of the url
        let dir_path = vec![fs_url.path(), &self.dir_path].join("/");
        RenamePathBuilder::new(destination_client, self.file_system_client.context.clone())
            .mode(PathRenameMode::Legacy)
            .rename_source(dir_path)
    }

    pub fn rename_if_not_exists<P>(&self, destination_path: P) -> RenamePathBuilder<Self>
    where
        P: Into<String>,
    {
        self.rename(destination_path)
            .if_match_condition(IfMatchCondition::NotMatch("*".to_string()))
    }

    pub fn delete<R>(&self, recursive: R) -> DeletePathBuilder<Self>
    where
        R: Into<Recursive>,
    {
        DeletePathBuilder::new(
            self.clone(),
            Some(recursive.into()),
            self.file_system_client.context.clone(),
        )
    }

    pub fn get_properties(&self) -> HeadPathBuilder<Self> {
        HeadPathBuilder::new(self.clone(), self.file_system_client.context.clone())
    }

    pub fn get_status(&self) -> HeadPathBuilder<Self> {
        HeadPathBuilder::new(self.clone(), self.file_system_client.context.clone())
            .action(PathGetPropertiesAction::GetStatus)
    }

    pub fn get_access_control_list(&self) -> HeadPathBuilder<Self> {
        HeadPathBuilder::new(self.clone(), self.file_system_client.context.clone())
            .action(PathGetPropertiesAction::GetAccessControl)
    }

    pub fn set_properties(&self, properties: impl Into<Properties>) -> PatchPathBuilder<Self> {
        PatchPathBuilder::new(self.clone(), self.file_system_client.context.clone())
            .properties(properties)
            .action(PathUpdateAction::SetProperties)
    }

    pub fn set_access_control_list(
        &self,
        acl: impl Into<AccessControlList>,
        recursive: bool,
    ) -> PatchPathBuilder<Self> {
        let builder =
            PatchPathBuilder::new(self.clone(), self.file_system_client.context.clone()).acl(acl);
        if recursive {
            builder.action(PathUpdateAction::SetAccessControlRecursive)
        } else {
            builder.action(PathUpdateAction::SetAccessControl)
        }
    }
}
