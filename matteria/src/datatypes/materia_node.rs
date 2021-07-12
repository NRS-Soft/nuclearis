#![cfg_attr(not(feature = "std"), no_std)]
#![allow(dead_code)]
use frame_support::codec::{Decode, Encode};
use frame_support::RuntimeDebug;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_std::vec::Vec;

pub fn convert_to_vec_u8(input: &str) -> Vec<u8> {
    input.as_bytes().to_vec()
}

#[derive(Encode, Decode)]
pub struct Material<Timestamp>{
    pub title: Vec<u8>,
    pub created_at: Timestamp,
    pub heat: Vec<u8>,
    pub documents: Vec<DocumentId>,
}

#[derive(Encode, Decode, Default, Clone, RuntimeDebug, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct Document {
    pub title_spa: Vec<u8>, 
    pub title_eng: Vec<u8>, 
    pub project: ProjectId,
    pub department: DepartmentId,
    pub document_type: TypeId,
    pub revision: Vec<RevisionId>,
    pub code: Vec<Vec<u8>>,
}

#[derive(Encode, Decode, Default, Clone, RuntimeDebug, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct Revision<AccountId, Timestamp> {
    pub ed_number: u8,
    pub rev_number: u8,
    pub ipfs_hash: Option<Vec<u8>>,
    pub state: RevisionState,
    pub created_by: Option<UserRevision<Timestamp, AccountId>>,
    pub reviewed_by: Option<Vec<UserRevision<Timestamp, AccountId>>>,
    pub approved_by: Option<Vec<UserRevision<Timestamp, AccountId>>>,
    pub controlled_copies: Option<Vec<ControlledCopies<AccountId, Timestamp>>>,
}

#[derive(Encode, Decode, Default, Clone, RuntimeDebug, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct User {
    pub name: Vec<u8>,
    pub position: Vec<PositionId>,
    pub email: Vec<u8>,
    pub department: DepartmentId,
    pub permissions: Vec<Permissions>,
    pub allowed_projects: Option<Vec<ProjectId>>,
    pub state: UserState,
}

#[derive(Encode, Decode, Default, Clone, RuntimeDebug, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct Position {
    pub name: Vec<u8>,
    pub superior: Option<PositionId>,
}


#[derive(Encode, Decode, Default, Clone, RuntimeDebug, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct ControlledCopies<AccountId, Timestamp> {
    pub recipient_name: Vec<u8>,
    pub recipient_email: Vec<u8>,
    pub date: Timestamp,
    pub created_by: AccountId,
}

#[derive(Encode, Decode, Default, Clone, RuntimeDebug, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct UserRevision<Timestamp, AccountId> {
    pub account: AccountId,
    pub name: Vec<u8>,
    pub timestamp: Timestamp,
    pub position: Vec<u8>,
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum UserState {
    Active,
    NotAvailable,
    Disabled,
}

impl Default for UserState {
    fn default() -> UserState {
        UserState::Active
    }
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum Permissions {
    // Document
    CreateDocument,
    CancelDocument,
    // Revisions
    CreateRevision,
    CreateEdition,
    // Material
    CreateMaterial,
    CloseMaterial,
    // Users
    CreateUser,
    ModifyUser,
    CreatePosition,
    ModifyPosition,
    DeletePosition,
    // Copies
    CreateControlledCopies,
    DownloadNonControlledCopies,
    // Project
    CreateProject,
    CloseProject,
    // Document Types
    CreateDocumentType,
    // Department
    CreateDepartment,
}

impl Default for Permissions {
    fn default() -> Permissions {
        Permissions::CreateDocument
    }
}

pub type TypeId = u32;
pub type ProjectId = u32;
pub type DepartmentId = u32;
pub type DocumentId = u32;
pub type RevisionId = u32;
pub type PositionId = u32;
pub type NextDocumentSequence = u16;

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum CodeType {
    Project,
    Type,
    Department,
    Dash,
    Year,
    Sequence,
    Edition,
    Revision,
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum ProjectState {
    Open,
    Closed,
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum RevisionState {
    EnElaboracion,
    EnRevision,
    EnAprobacion,
    Vigente,
    Superado,
    Anulado,
}

impl Default for RevisionState {
    fn default() -> RevisionState {
        RevisionState::EnElaboracion
    }
}

impl Default for ProjectState {
    fn default() -> ProjectState {
        ProjectState::Open
    }
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, Default, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct DocumentType {
    pub title_spa: Vec<u8>,
    pub title_eng: Vec<u8>,
    pub abbreviation: Vec<u8>,
    // If true only allowed for QSP Project.
    pub is_qsp: bool,
    // Departments that can be used in the creation of the documents code
    pub enabled_managments: Option<Vec<DepartmentId>>,
    pub base_document: Option<DocumentId>,
    // Viewer of documents 
    pub viewer: AuthConfig,
    // Creator of code and uploader of file
    pub creator: AuthConfig,
    // Reviewer of revision, can be None, when no reviewer is required 
    pub reviewer: Option<AuthConfig>,
    // Approver of revision, can be None, when no approver is required
    pub approver: Option<AuthConfig>,
    pub code_format: Vec<CodeType>,
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, Default, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct AuthConfig {
    pub position: Option<Vec<PositionId>>,
    // true means that all have to accept
    // false means that one of the positions have to accept.
    // The option is to enable passing to everyone
    pub forward_criteria: Option<bool>, 
    pub department: Option<Vec<DepartmentId>>,
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, Default, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct Project {
    pub title: Vec<u8>,
    pub abbreviation: Vec<u8>,
    pub purchase_order: Vec<u8>,
    pub client: Vec<u8>,
    pub contract: Vec<u8>,
    pub dossier: Vec<u8>,
    pub state: ProjectState,
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, Default, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct Department {
    pub title: Vec<u8>,
    pub abbreviation: Vec<u8>,
}