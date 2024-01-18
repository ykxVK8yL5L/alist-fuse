use std::ops;
use std::time::SystemTime;

use ::time::{format_description::well_known::Rfc3339, OffsetDateTime};
use serde::{Deserialize, Deserializer, Serialize};



#[derive(Debug, Clone)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}



#[derive(Debug, Clone, Deserialize)]
pub struct RefreshTokenResponse {
    pub code: u64,
    pub message: String,
    pub data: Token,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Token{
    pub token: String,
}


#[derive(Debug, Clone, Serialize)]
pub struct ListFileRequest<'a> {
    pub path: &'a str,
    pub password: &'a str,
    pub page: u64,
    pub per_page: u64,
    pub refresh: bool,
}


#[derive(Debug, Clone, Deserialize)]
pub struct ListFileResponse {
    pub code : u64,
    pub message : String,
    pub data: ListFileContentResponse,
}


#[derive(Debug, Clone, Deserialize)]
pub struct ListFileContentResponse {
    pub total : u64,
    pub readme : String,
    pub header : String,
    pub write : bool,
    pub provider : String,
    pub content: Vec<ResFile>,
}



#[derive(Debug, Clone,Serialize, Deserialize)]
pub struct ResFile {
    pub name: String,
    pub size: u64,
    pub is_dir: bool,
    pub created: DateTime,
    pub modified: DateTime,
    pub sign: String,
    pub thumb: String,
    pub hashinfo: String,
}


#[derive(Debug, Clone, Serialize)]
pub struct GetFileDownloadUrlRequest<'a> {
    pub path: &'a str,
    pub password: &'a str,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetFileDownloadUrlResponse {
    pub code: u64,
    pub message: String,
    pub data: DownloadFile,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DownloadFile {
    pub name: String,
    pub size: u64,
    pub is_dir: bool,
    pub created: DateTime,
    pub modified: DateTime,
    pub sign: String,
    pub thumb: String,
    pub hashinfo: String,
    pub raw_url: String,
    pub readme: String,
    pub header: String,
    pub provider: String,
}


#[derive(Debug, Clone, Deserialize)]
pub struct GetDriveResponse {
    pub total_size: u64,
    pub used_size: u64,
}


#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FileType {
    Folder,
    File,
}


#[derive(Debug, Clone,Serialize)]
pub struct DateTime(SystemTime);

impl DateTime {
    pub fn new(st: SystemTime) -> Self {
        Self(st)
    }
}

impl<'a> Deserialize<'a> for DateTime {
    fn deserialize<D: Deserializer<'a>>(deserializer: D) -> Result<Self, D::Error> {
        let dt = OffsetDateTime::parse(<&str>::deserialize(deserializer)?, &Rfc3339)
            .map_err(serde::de::Error::custom)?;
        Ok(Self(dt.into()))
    }
}

impl ops::Deref for DateTime {
    type Target = SystemTime;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone,Serialize, Deserialize)]
pub struct AlistFile {
    pub path: String,
    pub file: ResFile,
}



#[derive(Debug, Clone, Serialize)]
pub struct CreateFolderRequest<'a> {
    pub kind: &'a str,
    pub name: &'a str,
    pub parent_id: &'a str,
}


#[derive(Debug, Clone,Serialize, Deserialize)]
pub struct CreateFolderResponse{
    pub upload_type: String,
    pub file: AlistFile,
}

#[derive(Debug, Clone,Serialize, Deserialize)]
pub struct TaskResponse{
    pub task_id: String,
}



#[derive(Debug, Clone, Serialize)]
pub struct DelFileRequest {
    pub ids: Vec<String>,
}


#[derive(Debug, Clone, Serialize)]
pub struct MoveFileRequest {
    pub ids: Vec<String>,
    pub to: MoveTo,
}


#[derive(Debug, Clone, Serialize)]
pub struct MoveTo {
    pub parent_id: String,
}


#[derive(Debug, Clone, Serialize)]
pub struct RenameFileRequest<'a>{
    pub name: &'a str,
}


#[derive(Debug, Clone,Serialize, Deserialize)]
pub struct UploadRequest {
    pub kind: String,
    pub name: String,
    pub size: u64,
    pub hash: String,
    pub upload_type: String,
    pub objProvider:ObjProvider,
    pub parent_id: String,
}

#[derive(Debug, Clone,Serialize, Deserialize)]
pub struct ObjProvider {
    pub provider: String,
}

#[derive(Debug, Clone,Serialize, Deserialize)]
pub struct OssArgs {
    pub bucket: String,
    pub endpoint: String,
    pub access_key_id: String,
    pub access_key_secret: String,
    pub key: String,
    pub security_token: String,
}


#[derive(Debug, Clone,Serialize, Deserialize)]
pub struct CompleteMultipartUpload {
    pub Part: Vec<PartInfo>,
}

#[derive(Debug, Clone,Serialize, Deserialize)]
pub struct PartInfo {
    #[serde(flatten)]
    pub PartNumber: PartNumber,
    pub ETag: String,
}

#[derive(Debug, Clone,Serialize, Deserialize)]
pub struct PartNumber {
    pub PartNumber: u64,
}




#[derive(Debug, Clone,Serialize, Deserialize)]
pub struct UploadResponse {
    pub upload_type: String,
    pub resumable: Resumable,
    pub file: AlistFile,
}


#[derive(Debug, Clone,Serialize, Deserialize)]
pub struct Resumable {
    pub kind: String,
    pub provider: String,
    pub params: UploadParams,
}

#[derive(Debug, Clone,Serialize, Deserialize)]
pub struct UploadParams {
    pub access_key_id: String,
    pub access_key_secret: String,
    pub bucket: String,
    pub endpoint: String,
    pub expiration: String,
    pub key: String,
    pub security_token: String,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct InitiateMultipartUploadResult {
    pub Bucket: String,
    pub Key: String,
    pub UploadId: String,
}





impl AlistFile {
    pub fn new_root() -> Self {
        let now = SystemTime::now();
        let resf = ResFile{
            name: "root".to_string(),
            size: 0,
            is_dir:true,
            created: DateTime(now),
            modified: DateTime(now),
            sign: "".to_string(),
            thumb: "".to_string(),
            hashinfo: "".to_string(),
        };
        Self {
            path: "/".to_string(),
            file:resf,
        }
    }
}
