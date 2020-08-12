use some_proc_crate::SomeProcCrateStruct;
use some_proc_crate::SomeProcCrateEnum;

#[derive(SomeProcCrateStruct)]
struct SomeStruct {
    field0: i32,
    #[someattr]
    field1: &'static str,
    #[someattr]
    field2: String
}

#[derive(SomeProcCrateEnum)]
enum SomeEnum {
    Variant0,
    #[someattr]
    Variant1,
    #[someattr]
    Variant2
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_compile() {

    }
}