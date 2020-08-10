use some_proc_crate::SomeProcCrate;

#[derive(SomeProcCrate)]
struct SomeStruct {
    field0: i32,
    #[someattr]
    field1: &'static str,
    #[someattr]
    field2: String
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_compile() {

    }
}