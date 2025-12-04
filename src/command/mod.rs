pub mod login;
pub mod register;
pub mod changepassword;
pub mod bindemail;
pub mod resetpassword;

pub use changepassword::Changepassword;
pub use register::Register;
pub use login::Login;
pub use bindemail::Bindemail;
pub use resetpassword::Resetpassword;