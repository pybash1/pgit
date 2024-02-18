use crate::structures::Object;

#[derive(PartialEq)]
pub enum GetFileContentsReturnType {
    NoReturn,
    Size,
    Contents,
    Type,
}

pub fn get_file_contents(object_hash: String, return_type: GetFileContentsReturnType) -> String {
    let object = Object::new(object_hash);

    if return_type == GetFileContentsReturnType::Contents {
        object.get_contents()
    } else if return_type == GetFileContentsReturnType::Size {
        object.size.to_string()
    } else if return_type == GetFileContentsReturnType::Type {
        object.obj_type.to_string()
    } else {
        String::new()
    }
}
