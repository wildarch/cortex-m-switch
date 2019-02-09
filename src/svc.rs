#[macro_export]
macro_rules! svc {
    (0 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_0")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_0($($arg_name: u32)*);
            }
            cortex_m_switch_svc_0($($arg_val)*)
        }
    };
    (1 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_1")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_1($($arg_name: u32)*);
            }
            cortex_m_switch_svc_1($($arg_val)*)
        }
    };
    (2 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_2")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_2($($arg_name: u32)*);
            }
            cortex_m_switch_svc_2($($arg_val)*)
        }
    };
    (3 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_3")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_3($($arg_name: u32)*);
            }
            cortex_m_switch_svc_3($($arg_val)*)
        }
    };
    (4 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_4")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_4($($arg_name: u32)*);
            }
            cortex_m_switch_svc_4($($arg_val)*)
        }
    };
    (5 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_5")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_5($($arg_name: u32)*);
            }
            cortex_m_switch_svc_5($($arg_val)*)
        }
    };
    (6 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_6")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_6($($arg_name: u32)*);
            }
            cortex_m_switch_svc_6($($arg_val)*)
        }
    };
    (7 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_7")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_7($($arg_name: u32)*);
            }
            cortex_m_switch_svc_7($($arg_val)*)
        }
    };
    (8 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_8")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_8($($arg_name: u32)*);
            }
            cortex_m_switch_svc_8($($arg_val)*)
        }
    };
    (9 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_9")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_9($($arg_name: u32)*);
            }
            cortex_m_switch_svc_9($($arg_val)*)
        }
    };
    (10 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_10")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_10($($arg_name: u32)*);
            }
            cortex_m_switch_svc_10($($arg_val)*)
        }
    };
    (11 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_11")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_11($($arg_name: u32)*);
            }
            cortex_m_switch_svc_11($($arg_val)*)
        }
    };
    (12 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_12")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_12($($arg_name: u32)*);
            }
            cortex_m_switch_svc_12($($arg_val)*)
        }
    };
    (13 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_13")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_13($($arg_name: u32)*);
            }
            cortex_m_switch_svc_13($($arg_val)*)
        }
    };
    (14 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_14")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_14($($arg_name: u32)*);
            }
            cortex_m_switch_svc_14($($arg_val)*)
        }
    };
    (15 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_15")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_15($($arg_name: u32)*);
            }
            cortex_m_switch_svc_15($($arg_val)*)
        }
    };
    (16 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_16")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_16($($arg_name: u32)*);
            }
            cortex_m_switch_svc_16($($arg_val)*)
        }
    };
    (17 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_17")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_17($($arg_name: u32)*);
            }
            cortex_m_switch_svc_17($($arg_val)*)
        }
    };
    (18 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_18")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_18($($arg_name: u32)*);
            }
            cortex_m_switch_svc_18($($arg_val)*)
        }
    };
    (19 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_19")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_19($($arg_name: u32)*);
            }
            cortex_m_switch_svc_19($($arg_val)*)
        }
    };
    (20 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_20")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_20($($arg_name: u32)*);
            }
            cortex_m_switch_svc_20($($arg_val)*)
        }
    };
    (21 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_21")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_21($($arg_name: u32)*);
            }
            cortex_m_switch_svc_21($($arg_val)*)
        }
    };
    (22 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_22")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_22($($arg_name: u32)*);
            }
            cortex_m_switch_svc_22($($arg_val)*)
        }
    };
    (23 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_23")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_23($($arg_name: u32)*);
            }
            cortex_m_switch_svc_23($($arg_val)*)
        }
    };
    (24 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_24")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_24($($arg_name: u32)*);
            }
            cortex_m_switch_svc_24($($arg_val)*)
        }
    };
    (25 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_25")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_25($($arg_name: u32)*);
            }
            cortex_m_switch_svc_25($($arg_val)*)
        }
    };
    (26 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_26")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_26($($arg_name: u32)*);
            }
            cortex_m_switch_svc_26($($arg_val)*)
        }
    };
    (27 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_27")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_27($($arg_name: u32)*);
            }
            cortex_m_switch_svc_27($($arg_val)*)
        }
    };
    (28 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_28")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_28($($arg_name: u32)*);
            }
            cortex_m_switch_svc_28($($arg_val)*)
        }
    };
    (29 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_29")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_29($($arg_name: u32)*);
            }
            cortex_m_switch_svc_29($($arg_val)*)
        }
    };
    (30 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_30")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_30($($arg_name: u32)*);
            }
            cortex_m_switch_svc_30($($arg_val)*)
        }
    };
    (31 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_31")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_31($($arg_name: u32)*);
            }
            cortex_m_switch_svc_31($($arg_val)*)
        }
    };
    (32 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_32")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_32($($arg_name: u32)*);
            }
            cortex_m_switch_svc_32($($arg_val)*)
        }
    };
    (33 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_33")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_33($($arg_name: u32)*);
            }
            cortex_m_switch_svc_33($($arg_val)*)
        }
    };
    (34 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_34")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_34($($arg_name: u32)*);
            }
            cortex_m_switch_svc_34($($arg_val)*)
        }
    };
    (35 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_35")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_35($($arg_name: u32)*);
            }
            cortex_m_switch_svc_35($($arg_val)*)
        }
    };
    (36 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_36")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_36($($arg_name: u32)*);
            }
            cortex_m_switch_svc_36($($arg_val)*)
        }
    };
    (37 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_37")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_37($($arg_name: u32)*);
            }
            cortex_m_switch_svc_37($($arg_val)*)
        }
    };
    (38 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_38")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_38($($arg_name: u32)*);
            }
            cortex_m_switch_svc_38($($arg_val)*)
        }
    };
    (39 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_39")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_39($($arg_name: u32)*);
            }
            cortex_m_switch_svc_39($($arg_val)*)
        }
    };
    (40 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_40")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_40($($arg_name: u32)*);
            }
            cortex_m_switch_svc_40($($arg_val)*)
        }
    };
    (41 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_41")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_41($($arg_name: u32)*);
            }
            cortex_m_switch_svc_41($($arg_val)*)
        }
    };
    (42 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_42")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_42($($arg_name: u32)*);
            }
            cortex_m_switch_svc_42($($arg_val)*)
        }
    };
    (43 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_43")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_43($($arg_name: u32)*);
            }
            cortex_m_switch_svc_43($($arg_val)*)
        }
    };
    (44 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_44")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_44($($arg_name: u32)*);
            }
            cortex_m_switch_svc_44($($arg_val)*)
        }
    };
    (45 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_45")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_45($($arg_name: u32)*);
            }
            cortex_m_switch_svc_45($($arg_val)*)
        }
    };
    (46 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_46")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_46($($arg_name: u32)*);
            }
            cortex_m_switch_svc_46($($arg_val)*)
        }
    };
    (47 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_47")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_47($($arg_name: u32)*);
            }
            cortex_m_switch_svc_47($($arg_val)*)
        }
    };
    (48 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_48")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_48($($arg_name: u32)*);
            }
            cortex_m_switch_svc_48($($arg_val)*)
        }
    };
    (49 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_49")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_49($($arg_name: u32)*);
            }
            cortex_m_switch_svc_49($($arg_val)*)
        }
    };
    (50 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_50")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_50($($arg_name: u32)*);
            }
            cortex_m_switch_svc_50($($arg_val)*)
        }
    };
    (51 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_51")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_51($($arg_name: u32)*);
            }
            cortex_m_switch_svc_51($($arg_val)*)
        }
    };
    (52 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_52")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_52($($arg_name: u32)*);
            }
            cortex_m_switch_svc_52($($arg_val)*)
        }
    };
    (53 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_53")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_53($($arg_name: u32)*);
            }
            cortex_m_switch_svc_53($($arg_val)*)
        }
    };
    (54 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_54")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_54($($arg_name: u32)*);
            }
            cortex_m_switch_svc_54($($arg_val)*)
        }
    };
    (55 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_55")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_55($($arg_name: u32)*);
            }
            cortex_m_switch_svc_55($($arg_val)*)
        }
    };
    (56 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_56")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_56($($arg_name: u32)*);
            }
            cortex_m_switch_svc_56($($arg_val)*)
        }
    };
    (57 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_57")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_57($($arg_name: u32)*);
            }
            cortex_m_switch_svc_57($($arg_val)*)
        }
    };
    (58 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_58")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_58($($arg_name: u32)*);
            }
            cortex_m_switch_svc_58($($arg_val)*)
        }
    };
    (59 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_59")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_59($($arg_name: u32)*);
            }
            cortex_m_switch_svc_59($($arg_val)*)
        }
    };
    (60 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_60")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_60($($arg_name: u32)*);
            }
            cortex_m_switch_svc_60($($arg_val)*)
        }
    };
    (61 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_61")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_61($($arg_name: u32)*);
            }
            cortex_m_switch_svc_61($($arg_val)*)
        }
    };
    (62 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_62")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_62($($arg_name: u32)*);
            }
            cortex_m_switch_svc_62($($arg_val)*)
        }
    };
    (63 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_63")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_63($($arg_name: u32)*);
            }
            cortex_m_switch_svc_63($($arg_val)*)
        }
    };
    (64 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_64")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_64($($arg_name: u32)*);
            }
            cortex_m_switch_svc_64($($arg_val)*)
        }
    };
    (65 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_65")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_65($($arg_name: u32)*);
            }
            cortex_m_switch_svc_65($($arg_val)*)
        }
    };
    (66 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_66")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_66($($arg_name: u32)*);
            }
            cortex_m_switch_svc_66($($arg_val)*)
        }
    };
    (67 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_67")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_67($($arg_name: u32)*);
            }
            cortex_m_switch_svc_67($($arg_val)*)
        }
    };
    (68 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_68")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_68($($arg_name: u32)*);
            }
            cortex_m_switch_svc_68($($arg_val)*)
        }
    };
    (69 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_69")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_69($($arg_name: u32)*);
            }
            cortex_m_switch_svc_69($($arg_val)*)
        }
    };
    (70 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_70")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_70($($arg_name: u32)*);
            }
            cortex_m_switch_svc_70($($arg_val)*)
        }
    };
    (71 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_71")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_71($($arg_name: u32)*);
            }
            cortex_m_switch_svc_71($($arg_val)*)
        }
    };
    (72 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_72")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_72($($arg_name: u32)*);
            }
            cortex_m_switch_svc_72($($arg_val)*)
        }
    };
    (73 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_73")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_73($($arg_name: u32)*);
            }
            cortex_m_switch_svc_73($($arg_val)*)
        }
    };
    (74 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_74")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_74($($arg_name: u32)*);
            }
            cortex_m_switch_svc_74($($arg_val)*)
        }
    };
    (75 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_75")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_75($($arg_name: u32)*);
            }
            cortex_m_switch_svc_75($($arg_val)*)
        }
    };
    (76 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_76")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_76($($arg_name: u32)*);
            }
            cortex_m_switch_svc_76($($arg_val)*)
        }
    };
    (77 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_77")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_77($($arg_name: u32)*);
            }
            cortex_m_switch_svc_77($($arg_val)*)
        }
    };
    (78 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_78")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_78($($arg_name: u32)*);
            }
            cortex_m_switch_svc_78($($arg_val)*)
        }
    };
    (79 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_79")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_79($($arg_name: u32)*);
            }
            cortex_m_switch_svc_79($($arg_val)*)
        }
    };
    (80 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_80")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_80($($arg_name: u32)*);
            }
            cortex_m_switch_svc_80($($arg_val)*)
        }
    };
    (81 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_81")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_81($($arg_name: u32)*);
            }
            cortex_m_switch_svc_81($($arg_val)*)
        }
    };
    (82 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_82")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_82($($arg_name: u32)*);
            }
            cortex_m_switch_svc_82($($arg_val)*)
        }
    };
    (83 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_83")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_83($($arg_name: u32)*);
            }
            cortex_m_switch_svc_83($($arg_val)*)
        }
    };
    (84 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_84")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_84($($arg_name: u32)*);
            }
            cortex_m_switch_svc_84($($arg_val)*)
        }
    };
    (85 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_85")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_85($($arg_name: u32)*);
            }
            cortex_m_switch_svc_85($($arg_val)*)
        }
    };
    (86 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_86")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_86($($arg_name: u32)*);
            }
            cortex_m_switch_svc_86($($arg_val)*)
        }
    };
    (87 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_87")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_87($($arg_name: u32)*);
            }
            cortex_m_switch_svc_87($($arg_val)*)
        }
    };
    (88 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_88")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_88($($arg_name: u32)*);
            }
            cortex_m_switch_svc_88($($arg_val)*)
        }
    };
    (89 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_89")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_89($($arg_name: u32)*);
            }
            cortex_m_switch_svc_89($($arg_val)*)
        }
    };
    (90 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_90")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_90($($arg_name: u32)*);
            }
            cortex_m_switch_svc_90($($arg_val)*)
        }
    };
    (91 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_91")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_91($($arg_name: u32)*);
            }
            cortex_m_switch_svc_91($($arg_val)*)
        }
    };
    (92 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_92")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_92($($arg_name: u32)*);
            }
            cortex_m_switch_svc_92($($arg_val)*)
        }
    };
    (93 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_93")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_93($($arg_name: u32)*);
            }
            cortex_m_switch_svc_93($($arg_val)*)
        }
    };
    (94 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_94")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_94($($arg_name: u32)*);
            }
            cortex_m_switch_svc_94($($arg_val)*)
        }
    };
    (95 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_95")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_95($($arg_name: u32)*);
            }
            cortex_m_switch_svc_95($($arg_val)*)
        }
    };
    (96 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_96")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_96($($arg_name: u32)*);
            }
            cortex_m_switch_svc_96($($arg_val)*)
        }
    };
    (97 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_97")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_97($($arg_name: u32)*);
            }
            cortex_m_switch_svc_97($($arg_val)*)
        }
    };
    (98 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_98")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_98($($arg_name: u32)*);
            }
            cortex_m_switch_svc_98($($arg_val)*)
        }
    };
    (99 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_99")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_99($($arg_name: u32)*);
            }
            cortex_m_switch_svc_99($($arg_val)*)
        }
    };
    (100 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_100")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_100($($arg_name: u32)*);
            }
            cortex_m_switch_svc_100($($arg_val)*)
        }
    };
    (101 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_101")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_101($($arg_name: u32)*);
            }
            cortex_m_switch_svc_101($($arg_val)*)
        }
    };
    (102 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_102")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_102($($arg_name: u32)*);
            }
            cortex_m_switch_svc_102($($arg_val)*)
        }
    };
    (103 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_103")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_103($($arg_name: u32)*);
            }
            cortex_m_switch_svc_103($($arg_val)*)
        }
    };
    (104 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_104")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_104($($arg_name: u32)*);
            }
            cortex_m_switch_svc_104($($arg_val)*)
        }
    };
    (105 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_105")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_105($($arg_name: u32)*);
            }
            cortex_m_switch_svc_105($($arg_val)*)
        }
    };
    (106 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_106")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_106($($arg_name: u32)*);
            }
            cortex_m_switch_svc_106($($arg_val)*)
        }
    };
    (107 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_107")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_107($($arg_name: u32)*);
            }
            cortex_m_switch_svc_107($($arg_val)*)
        }
    };
    (108 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_108")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_108($($arg_name: u32)*);
            }
            cortex_m_switch_svc_108($($arg_val)*)
        }
    };
    (109 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_109")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_109($($arg_name: u32)*);
            }
            cortex_m_switch_svc_109($($arg_val)*)
        }
    };
    (110 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_110")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_110($($arg_name: u32)*);
            }
            cortex_m_switch_svc_110($($arg_val)*)
        }
    };
    (111 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_111")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_111($($arg_name: u32)*);
            }
            cortex_m_switch_svc_111($($arg_val)*)
        }
    };
    (112 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_112")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_112($($arg_name: u32)*);
            }
            cortex_m_switch_svc_112($($arg_val)*)
        }
    };
    (113 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_113")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_113($($arg_name: u32)*);
            }
            cortex_m_switch_svc_113($($arg_val)*)
        }
    };
    (114 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_114")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_114($($arg_name: u32)*);
            }
            cortex_m_switch_svc_114($($arg_val)*)
        }
    };
    (115 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_115")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_115($($arg_name: u32)*);
            }
            cortex_m_switch_svc_115($($arg_val)*)
        }
    };
    (116 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_116")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_116($($arg_name: u32)*);
            }
            cortex_m_switch_svc_116($($arg_val)*)
        }
    };
    (117 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_117")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_117($($arg_name: u32)*);
            }
            cortex_m_switch_svc_117($($arg_val)*)
        }
    };
    (118 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_118")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_118($($arg_name: u32)*);
            }
            cortex_m_switch_svc_118($($arg_val)*)
        }
    };
    (119 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_119")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_119($($arg_name: u32)*);
            }
            cortex_m_switch_svc_119($($arg_val)*)
        }
    };
    (120 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_120")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_120($($arg_name: u32)*);
            }
            cortex_m_switch_svc_120($($arg_val)*)
        }
    };
    (121 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_121")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_121($($arg_name: u32)*);
            }
            cortex_m_switch_svc_121($($arg_val)*)
        }
    };
    (122 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_122")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_122($($arg_name: u32)*);
            }
            cortex_m_switch_svc_122($($arg_val)*)
        }
    };
    (123 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_123")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_123($($arg_name: u32)*);
            }
            cortex_m_switch_svc_123($($arg_val)*)
        }
    };
    (124 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_124")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_124($($arg_name: u32)*);
            }
            cortex_m_switch_svc_124($($arg_val)*)
        }
    };
    (125 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_125")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_125($($arg_name: u32)*);
            }
            cortex_m_switch_svc_125($($arg_val)*)
        }
    };
    (126 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_126")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_126($($arg_name: u32)*);
            }
            cortex_m_switch_svc_126($($arg_val)*)
        }
    };
    (127 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_127")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_127($($arg_name: u32)*);
            }
            cortex_m_switch_svc_127($($arg_val)*)
        }
    };
    (128 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_128")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_128($($arg_name: u32)*);
            }
            cortex_m_switch_svc_128($($arg_val)*)
        }
    };
    (129 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_129")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_129($($arg_name: u32)*);
            }
            cortex_m_switch_svc_129($($arg_val)*)
        }
    };
    (130 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_130")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_130($($arg_name: u32)*);
            }
            cortex_m_switch_svc_130($($arg_val)*)
        }
    };
    (131 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_131")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_131($($arg_name: u32)*);
            }
            cortex_m_switch_svc_131($($arg_val)*)
        }
    };
    (132 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_132")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_132($($arg_name: u32)*);
            }
            cortex_m_switch_svc_132($($arg_val)*)
        }
    };
    (133 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_133")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_133($($arg_name: u32)*);
            }
            cortex_m_switch_svc_133($($arg_val)*)
        }
    };
    (134 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_134")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_134($($arg_name: u32)*);
            }
            cortex_m_switch_svc_134($($arg_val)*)
        }
    };
    (135 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_135")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_135($($arg_name: u32)*);
            }
            cortex_m_switch_svc_135($($arg_val)*)
        }
    };
    (136 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_136")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_136($($arg_name: u32)*);
            }
            cortex_m_switch_svc_136($($arg_val)*)
        }
    };
    (137 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_137")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_137($($arg_name: u32)*);
            }
            cortex_m_switch_svc_137($($arg_val)*)
        }
    };
    (138 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_138")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_138($($arg_name: u32)*);
            }
            cortex_m_switch_svc_138($($arg_val)*)
        }
    };
    (139 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_139")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_139($($arg_name: u32)*);
            }
            cortex_m_switch_svc_139($($arg_val)*)
        }
    };
    (140 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_140")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_140($($arg_name: u32)*);
            }
            cortex_m_switch_svc_140($($arg_val)*)
        }
    };
    (141 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_141")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_141($($arg_name: u32)*);
            }
            cortex_m_switch_svc_141($($arg_val)*)
        }
    };
    (142 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_142")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_142($($arg_name: u32)*);
            }
            cortex_m_switch_svc_142($($arg_val)*)
        }
    };
    (143 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_143")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_143($($arg_name: u32)*);
            }
            cortex_m_switch_svc_143($($arg_val)*)
        }
    };
    (144 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_144")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_144($($arg_name: u32)*);
            }
            cortex_m_switch_svc_144($($arg_val)*)
        }
    };
    (145 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_145")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_145($($arg_name: u32)*);
            }
            cortex_m_switch_svc_145($($arg_val)*)
        }
    };
    (146 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_146")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_146($($arg_name: u32)*);
            }
            cortex_m_switch_svc_146($($arg_val)*)
        }
    };
    (147 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_147")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_147($($arg_name: u32)*);
            }
            cortex_m_switch_svc_147($($arg_val)*)
        }
    };
    (148 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_148")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_148($($arg_name: u32)*);
            }
            cortex_m_switch_svc_148($($arg_val)*)
        }
    };
    (149 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_149")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_149($($arg_name: u32)*);
            }
            cortex_m_switch_svc_149($($arg_val)*)
        }
    };
    (150 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_150")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_150($($arg_name: u32)*);
            }
            cortex_m_switch_svc_150($($arg_val)*)
        }
    };
    (151 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_151")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_151($($arg_name: u32)*);
            }
            cortex_m_switch_svc_151($($arg_val)*)
        }
    };
    (152 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_152")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_152($($arg_name: u32)*);
            }
            cortex_m_switch_svc_152($($arg_val)*)
        }
    };
    (153 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_153")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_153($($arg_name: u32)*);
            }
            cortex_m_switch_svc_153($($arg_val)*)
        }
    };
    (154 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_154")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_154($($arg_name: u32)*);
            }
            cortex_m_switch_svc_154($($arg_val)*)
        }
    };
    (155 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_155")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_155($($arg_name: u32)*);
            }
            cortex_m_switch_svc_155($($arg_val)*)
        }
    };
    (156 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_156")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_156($($arg_name: u32)*);
            }
            cortex_m_switch_svc_156($($arg_val)*)
        }
    };
    (157 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_157")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_157($($arg_name: u32)*);
            }
            cortex_m_switch_svc_157($($arg_val)*)
        }
    };
    (158 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_158")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_158($($arg_name: u32)*);
            }
            cortex_m_switch_svc_158($($arg_val)*)
        }
    };
    (159 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_159")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_159($($arg_name: u32)*);
            }
            cortex_m_switch_svc_159($($arg_val)*)
        }
    };
    (160 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_160")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_160($($arg_name: u32)*);
            }
            cortex_m_switch_svc_160($($arg_val)*)
        }
    };
    (161 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_161")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_161($($arg_name: u32)*);
            }
            cortex_m_switch_svc_161($($arg_val)*)
        }
    };
    (162 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_162")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_162($($arg_name: u32)*);
            }
            cortex_m_switch_svc_162($($arg_val)*)
        }
    };
    (163 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_163")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_163($($arg_name: u32)*);
            }
            cortex_m_switch_svc_163($($arg_val)*)
        }
    };
    (164 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_164")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_164($($arg_name: u32)*);
            }
            cortex_m_switch_svc_164($($arg_val)*)
        }
    };
    (165 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_165")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_165($($arg_name: u32)*);
            }
            cortex_m_switch_svc_165($($arg_val)*)
        }
    };
    (166 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_166")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_166($($arg_name: u32)*);
            }
            cortex_m_switch_svc_166($($arg_val)*)
        }
    };
    (167 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_167")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_167($($arg_name: u32)*);
            }
            cortex_m_switch_svc_167($($arg_val)*)
        }
    };
    (168 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_168")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_168($($arg_name: u32)*);
            }
            cortex_m_switch_svc_168($($arg_val)*)
        }
    };
    (169 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_169")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_169($($arg_name: u32)*);
            }
            cortex_m_switch_svc_169($($arg_val)*)
        }
    };
    (170 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_170")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_170($($arg_name: u32)*);
            }
            cortex_m_switch_svc_170($($arg_val)*)
        }
    };
    (171 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_171")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_171($($arg_name: u32)*);
            }
            cortex_m_switch_svc_171($($arg_val)*)
        }
    };
    (172 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_172")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_172($($arg_name: u32)*);
            }
            cortex_m_switch_svc_172($($arg_val)*)
        }
    };
    (173 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_173")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_173($($arg_name: u32)*);
            }
            cortex_m_switch_svc_173($($arg_val)*)
        }
    };
    (174 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_174")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_174($($arg_name: u32)*);
            }
            cortex_m_switch_svc_174($($arg_val)*)
        }
    };
    (175 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_175")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_175($($arg_name: u32)*);
            }
            cortex_m_switch_svc_175($($arg_val)*)
        }
    };
    (176 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_176")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_176($($arg_name: u32)*);
            }
            cortex_m_switch_svc_176($($arg_val)*)
        }
    };
    (177 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_177")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_177($($arg_name: u32)*);
            }
            cortex_m_switch_svc_177($($arg_val)*)
        }
    };
    (178 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_178")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_178($($arg_name: u32)*);
            }
            cortex_m_switch_svc_178($($arg_val)*)
        }
    };
    (179 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_179")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_179($($arg_name: u32)*);
            }
            cortex_m_switch_svc_179($($arg_val)*)
        }
    };
    (180 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_180")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_180($($arg_name: u32)*);
            }
            cortex_m_switch_svc_180($($arg_val)*)
        }
    };
    (181 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_181")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_181($($arg_name: u32)*);
            }
            cortex_m_switch_svc_181($($arg_val)*)
        }
    };
    (182 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_182")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_182($($arg_name: u32)*);
            }
            cortex_m_switch_svc_182($($arg_val)*)
        }
    };
    (183 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_183")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_183($($arg_name: u32)*);
            }
            cortex_m_switch_svc_183($($arg_val)*)
        }
    };
    (184 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_184")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_184($($arg_name: u32)*);
            }
            cortex_m_switch_svc_184($($arg_val)*)
        }
    };
    (185 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_185")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_185($($arg_name: u32)*);
            }
            cortex_m_switch_svc_185($($arg_val)*)
        }
    };
    (186 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_186")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_186($($arg_name: u32)*);
            }
            cortex_m_switch_svc_186($($arg_val)*)
        }
    };
    (187 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_187")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_187($($arg_name: u32)*);
            }
            cortex_m_switch_svc_187($($arg_val)*)
        }
    };
    (188 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_188")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_188($($arg_name: u32)*);
            }
            cortex_m_switch_svc_188($($arg_val)*)
        }
    };
    (189 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_189")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_189($($arg_name: u32)*);
            }
            cortex_m_switch_svc_189($($arg_val)*)
        }
    };
    (190 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_190")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_190($($arg_name: u32)*);
            }
            cortex_m_switch_svc_190($($arg_val)*)
        }
    };
    (191 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_191")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_191($($arg_name: u32)*);
            }
            cortex_m_switch_svc_191($($arg_val)*)
        }
    };
    (192 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_192")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_192($($arg_name: u32)*);
            }
            cortex_m_switch_svc_192($($arg_val)*)
        }
    };
    (193 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_193")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_193($($arg_name: u32)*);
            }
            cortex_m_switch_svc_193($($arg_val)*)
        }
    };
    (194 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_194")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_194($($arg_name: u32)*);
            }
            cortex_m_switch_svc_194($($arg_val)*)
        }
    };
    (195 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_195")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_195($($arg_name: u32)*);
            }
            cortex_m_switch_svc_195($($arg_val)*)
        }
    };
    (196 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_196")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_196($($arg_name: u32)*);
            }
            cortex_m_switch_svc_196($($arg_val)*)
        }
    };
    (197 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_197")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_197($($arg_name: u32)*);
            }
            cortex_m_switch_svc_197($($arg_val)*)
        }
    };
    (198 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_198")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_198($($arg_name: u32)*);
            }
            cortex_m_switch_svc_198($($arg_val)*)
        }
    };
    (199 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_199")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_199($($arg_name: u32)*);
            }
            cortex_m_switch_svc_199($($arg_val)*)
        }
    };
    (200 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_200")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_200($($arg_name: u32)*);
            }
            cortex_m_switch_svc_200($($arg_val)*)
        }
    };
    (201 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_201")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_201($($arg_name: u32)*);
            }
            cortex_m_switch_svc_201($($arg_val)*)
        }
    };
    (202 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_202")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_202($($arg_name: u32)*);
            }
            cortex_m_switch_svc_202($($arg_val)*)
        }
    };
    (203 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_203")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_203($($arg_name: u32)*);
            }
            cortex_m_switch_svc_203($($arg_val)*)
        }
    };
    (204 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_204")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_204($($arg_name: u32)*);
            }
            cortex_m_switch_svc_204($($arg_val)*)
        }
    };
    (205 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_205")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_205($($arg_name: u32)*);
            }
            cortex_m_switch_svc_205($($arg_val)*)
        }
    };
    (206 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_206")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_206($($arg_name: u32)*);
            }
            cortex_m_switch_svc_206($($arg_val)*)
        }
    };
    (207 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_207")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_207($($arg_name: u32)*);
            }
            cortex_m_switch_svc_207($($arg_val)*)
        }
    };
    (208 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_208")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_208($($arg_name: u32)*);
            }
            cortex_m_switch_svc_208($($arg_val)*)
        }
    };
    (209 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_209")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_209($($arg_name: u32)*);
            }
            cortex_m_switch_svc_209($($arg_val)*)
        }
    };
    (210 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_210")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_210($($arg_name: u32)*);
            }
            cortex_m_switch_svc_210($($arg_val)*)
        }
    };
    (211 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_211")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_211($($arg_name: u32)*);
            }
            cortex_m_switch_svc_211($($arg_val)*)
        }
    };
    (212 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_212")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_212($($arg_name: u32)*);
            }
            cortex_m_switch_svc_212($($arg_val)*)
        }
    };
    (213 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_213")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_213($($arg_name: u32)*);
            }
            cortex_m_switch_svc_213($($arg_val)*)
        }
    };
    (214 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_214")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_214($($arg_name: u32)*);
            }
            cortex_m_switch_svc_214($($arg_val)*)
        }
    };
    (215 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_215")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_215($($arg_name: u32)*);
            }
            cortex_m_switch_svc_215($($arg_val)*)
        }
    };
    (216 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_216")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_216($($arg_name: u32)*);
            }
            cortex_m_switch_svc_216($($arg_val)*)
        }
    };
    (217 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_217")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_217($($arg_name: u32)*);
            }
            cortex_m_switch_svc_217($($arg_val)*)
        }
    };
    (218 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_218")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_218($($arg_name: u32)*);
            }
            cortex_m_switch_svc_218($($arg_val)*)
        }
    };
    (219 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_219")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_219($($arg_name: u32)*);
            }
            cortex_m_switch_svc_219($($arg_val)*)
        }
    };
    (220 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_220")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_220($($arg_name: u32)*);
            }
            cortex_m_switch_svc_220($($arg_val)*)
        }
    };
    (221 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_221")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_221($($arg_name: u32)*);
            }
            cortex_m_switch_svc_221($($arg_val)*)
        }
    };
    (222 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_222")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_222($($arg_name: u32)*);
            }
            cortex_m_switch_svc_222($($arg_val)*)
        }
    };
    (223 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_223")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_223($($arg_name: u32)*);
            }
            cortex_m_switch_svc_223($($arg_val)*)
        }
    };
    (224 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_224")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_224($($arg_name: u32)*);
            }
            cortex_m_switch_svc_224($($arg_val)*)
        }
    };
    (225 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_225")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_225($($arg_name: u32)*);
            }
            cortex_m_switch_svc_225($($arg_val)*)
        }
    };
    (226 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_226")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_226($($arg_name: u32)*);
            }
            cortex_m_switch_svc_226($($arg_val)*)
        }
    };
    (227 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_227")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_227($($arg_name: u32)*);
            }
            cortex_m_switch_svc_227($($arg_val)*)
        }
    };
    (228 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_228")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_228($($arg_name: u32)*);
            }
            cortex_m_switch_svc_228($($arg_val)*)
        }
    };
    (229 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_229")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_229($($arg_name: u32)*);
            }
            cortex_m_switch_svc_229($($arg_val)*)
        }
    };
    (230 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_230")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_230($($arg_name: u32)*);
            }
            cortex_m_switch_svc_230($($arg_val)*)
        }
    };
    (231 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_231")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_231($($arg_name: u32)*);
            }
            cortex_m_switch_svc_231($($arg_val)*)
        }
    };
    (232 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_232")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_232($($arg_name: u32)*);
            }
            cortex_m_switch_svc_232($($arg_val)*)
        }
    };
    (233 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_233")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_233($($arg_name: u32)*);
            }
            cortex_m_switch_svc_233($($arg_val)*)
        }
    };
    (234 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_234")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_234($($arg_name: u32)*);
            }
            cortex_m_switch_svc_234($($arg_val)*)
        }
    };
    (235 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_235")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_235($($arg_name: u32)*);
            }
            cortex_m_switch_svc_235($($arg_val)*)
        }
    };
    (236 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_236")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_236($($arg_name: u32)*);
            }
            cortex_m_switch_svc_236($($arg_val)*)
        }
    };
    (237 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_237")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_237($($arg_name: u32)*);
            }
            cortex_m_switch_svc_237($($arg_val)*)
        }
    };
    (238 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_238")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_238($($arg_name: u32)*);
            }
            cortex_m_switch_svc_238($($arg_val)*)
        }
    };
    (239 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_239")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_239($($arg_name: u32)*);
            }
            cortex_m_switch_svc_239($($arg_val)*)
        }
    };
    (240 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_240")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_240($($arg_name: u32)*);
            }
            cortex_m_switch_svc_240($($arg_val)*)
        }
    };
    (241 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_241")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_241($($arg_name: u32)*);
            }
            cortex_m_switch_svc_241($($arg_val)*)
        }
    };
    (242 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_242")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_242($($arg_name: u32)*);
            }
            cortex_m_switch_svc_242($($arg_val)*)
        }
    };
    (243 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_243")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_243($($arg_name: u32)*);
            }
            cortex_m_switch_svc_243($($arg_val)*)
        }
    };
    (244 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_244")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_244($($arg_name: u32)*);
            }
            cortex_m_switch_svc_244($($arg_val)*)
        }
    };
    (245 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_245")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_245($($arg_name: u32)*);
            }
            cortex_m_switch_svc_245($($arg_val)*)
        }
    };
    (246 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_246")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_246($($arg_name: u32)*);
            }
            cortex_m_switch_svc_246($($arg_val)*)
        }
    };
    (247 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_247")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_247($($arg_name: u32)*);
            }
            cortex_m_switch_svc_247($($arg_val)*)
        }
    };
    (248 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_248")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_248($($arg_name: u32)*);
            }
            cortex_m_switch_svc_248($($arg_val)*)
        }
    };
    (249 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_249")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_249($($arg_name: u32)*);
            }
            cortex_m_switch_svc_249($($arg_val)*)
        }
    };
    (250 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_250")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_250($($arg_name: u32)*);
            }
            cortex_m_switch_svc_250($($arg_val)*)
        }
    };
    (251 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_251")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_251($($arg_name: u32)*);
            }
            cortex_m_switch_svc_251($($arg_val)*)
        }
    };
    (252 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_252")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_252($($arg_name: u32)*);
            }
            cortex_m_switch_svc_252($($arg_val)*)
        }
    };
    (253 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_253")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_253($($arg_name: u32)*);
            }
            cortex_m_switch_svc_253($($arg_val)*)
        }
    };
    (254 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_254")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_254($($arg_name: u32)*);
            }
            cortex_m_switch_svc_254($($arg_val)*)
        }
    };
    (255 $(, $arg_name:ident: $arg_val:expr)*) => {
        {
            #[link(name = "svc_255")]
            extern "C" {
                #[allow(dead_code)]
                pub fn cortex_m_switch_svc_255($($arg_name: u32)*);
            }
            cortex_m_switch_svc_255($($arg_val)*)
        }
    };
}
