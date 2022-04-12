#[cfg(test)]
mod test {

    #[derive(Debug, knuffel::Decode)]
    struct SomeDocument {
        #[knuffel(child)]
        _some_node: SomeEnum,
    }

    #[derive(Debug, knuffel::Decode)]
    enum SomeEnum {
        SomeVariant(SomeVariant),
        AnotherVariant(AnotherVariant),
    }

    #[derive(Debug, knuffel::Decode)]
    struct SomeVariant {}

    #[derive(Debug, knuffel::Decode)]
    struct AnotherVariant {}

    #[test]
    fn test_decode_node() {
        let r = knuffel::parse::<SomeDocument>("", "some-node");
        assert!(r.is_ok(), "{:#?}", r.unwrap_err());
    }


    #[test]
    fn test_decode_variant() {
        let r = knuffel::parse::<SomeDocument>("", "some-variant");
        assert!(r.is_ok(), "{:#?}", r.unwrap_err());
    }
}
