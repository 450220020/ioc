# ioc
rust new  ioc


///#[autowired]
/// static defsingle: Option<EntityObj> = None;

/// // 展开
/// // macro_rules! defsingle {
/// //     () => {
/// //         single_get!( "defsingle", EntityObj)
/// //     };
/// // }
/// 这里是注入，后面还需要撸全局扫描注册组件,写法应该几乎一致，同样可用的内容还有这几个crate  singlemap,iocmap,aop,extends-rs

/// #[test]
/// fn test_one() {
///    single_add!(
///        "defsingle",
///        Box::new(EntityObj {
///            key: "hhhhh".to_string()
///        })
///    );
///    println!("注入后获取值 :{:?}", defsingle!().key);
/// }