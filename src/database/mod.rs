mod space_group_table;

pub use self::space_group_table::LookUpSpaceGroup;
pub(crate) use space_group_table::DEFAULT_SPACE_GROUP_SYMBOLS;
pub(crate) use space_group_table::FULL_SPACE_GROUP_SYMBOLS;

/// Full list of space group hall symbols
#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum SpaceGroupHallSymbol {
    P_1,
    MP_1,
    P_2y,
    P_2,
    P_2x,
    P_2yb,
    P_2c,
    P_2xa,
    C_2y,
    A_2y,
    I_2y,
    A_2,
    B_2,
    I_2,
    B_2x,
    C_2x,
    I_2x,
    P_M2y,
    P_M2,
    P_M2x,
    P_M2yc,
    P_M2yac,
    P_M2ya,
    P_M2a,
    P_M2ab,
    P_M2b,
    P_M2xb,
    P_M2xbc,
    P_M2xc,
    C_M2y,
    A_M2y,
    I_M2y,
    A_M2,
    B_M2,
    I_M2,
    B_M2x,
    C_M2x,
    I_M2x,
    C_M2yc,
    A_M2yac,
    I_M2ya,
    A_M2ya,
    C_M2ybc,
    I_M2yc,
    A_M2a,
    B_M2bc,
    I_M2b,
    B_M2b,
    A_M2ac,
    I_M2a,
    B_M2xb,
    C_M2xbc,
    I_M2xc,
    C_M2xc,
    B_M2xbc,
    I_M2xb,
    MP_2y,
    MP_2,
    MP_2x,
    MP_2yb,
    MP_2c,
    MP_2xa,
    MC_2y,
    MA_2y,
    MI_2y,
    MA_2,
    MB_2,
    MI_2,
    MB_2x,
    MC_2x,
    MI_2x,
    MP_2yc,
    MP_2yac,
    MP_2ya,
    MP_2a,
    MP_2ab,
    MP_2b,
    MP_2xb,
    MP_2xbc,
    MP_2xc,
    MP_2ybc,
    MP_2yn,
    MP_2yab,
    MP_2ac,
    MP_2n,
    MP_2bc,
    MP_2xab,
    MP_2xn,
    MP_2xac,
    MC_2yc,
    MA_2yac,
    MI_2ya,
    MA_2ya,
    MC_2ybc,
    MI_2yc,
    MA_2a,
    MB_2bc,
    MI_2b,
    MB_2b,
    MA_2ac,
    MI_2a,
    MB_2xb,
    MC_2xbc,
    MI_2xc,
    MC_2xc,
    MB_2xbc,
    MI_2xb,
    P_2_2,
    P_2c_2,
    P_2a_2a,
    P_2_2b,
    P_2_2ab,
    P_2bc_2,
    P_2ac_2ac,
    P_2ac_2ab,
    C_2c_2,
    A_2a_2a,
    B_2_2b,
    C_2_2,
    A_2_2,
    B_2_2,
    F_2_2,
    I_2_2,
    I_2b_2c,
    P_2_M2,
    P_M2_2,
    P_M2_M2,
    P_2c_M2,
    P_2c_M2c,
    P_M2a_2a,
    P_M2_2a,
    P_M2_M2b,
    P_M2b_M2,
    P_2_M2c,
    P_M2a_2,
    P_M2b_M2b,
    P_2_M2a,
    P_2_M2b,
    P_M2b_2,
    P_M2c_2,
    P_M2c_M2c,
    P_M2a_M2a,
    P_2c_M2ac,
    P_2c_M2b,
    P_M2b_2a,
    P_M2ac_2a,
    P_M2bc_M2c,
    P_M2a_M2ab,
    P_2_M2bc,
    P_2_M2ac,
    P_M2ac_2,
    P_M2ab_2,
    P_M2ab_M2ab,
    P_M2bc_M2bc,
    P_2ac_M2,
    P_2bc_M2bc,
    P_M2ab_2ab,
    P_M2_2ac,
    P_M2_M2bc,
    P_M2ab_M2,
    P_2_M2ab,
    P_M2bc_2,
    P_M2ac_M2ac,
    P_2c_M2n,
    P_2c_M2ab,
    P_M2bc_2a,
    P_M2n_2a,
    P_M2n_M2ac,
    P_M2ac_M2n,
    P_2_M2n,
    P_M2n_2,
    P_M2n_M2n,
    C_2_M2,
    A_M2_2,
    B_M2_M2,
    C_2c_M2,
    C_2c_M2c,
    A_M2a_2a,
    A_M2_2a,
    B_M2_M2b,
    B_M2b_M2,
    C_2_M2c,
    A_M2a_2,
    B_M2b_M2b,
    A_2_M2,
    B_2_M2,
    B_M2_2,
    C_M2_2,
    C_M2_M2,
    A_M2_M2,
    A_2_M2c,
    B_2_M2c,
    B_M2c_2,
    C_M2b_2,
    C_M2b_M2b,
    A_M2c_M2c,
    A_2_M2a,
    B_2_M2b,
    B_M2b_2,
    C_M2c_2,
    C_M2c_M2c,
    A_M2a_M2a,
    A_2_M2ac,
    B_2_M2bc,
    B_M2bc_2,
    C_M2bc_2,
    C_M2bc_M2bc,
    A_M2ac_M2ac,
    F_2_M2,
    F_M2_2,
    F_M2_M2,
    F_2_M2d,
    F_M2d_2,
    F_M2d_M2d,
    I_2_M2,
    I_M2_2,
    I_M2_M2,
    I_2_M2c,
    I_M2a_2,
    I_M2b_M2b,
    I_2_M2a,
    I_2_M2b,
    I_M2b_2,
    I_M2c_2,
    I_M2c_M2c,
    I_M2a_M2a,
    MP_2_2,
    P_2_2_M1n,
    MP_2ab_2bc,
    MP_2_2c,
    MP_2a_2,
    MP_2b_2b,
    P_2_2_M1ab,
    MP_2ab_2b,
    P_2_2_M1bc,
    MP_2b_2bc,
    P_2_2_M1ac,
    MP_2a_2c,
    MP_2a_2a,
    MP_2b_2,
    MP_2_2b,
    MP_2c_2c,
    MP_2c_2,
    MP_2_2a,
    MP_2a_2bc,
    MP_2b_2n,
    MP_2n_2b,
    MP_2ab_2c,
    MP_2ab_2n,
    MP_2n_2bc,
    MP_2ac_2,
    MP_2bc_2bc,
    MP_2ab_2ab,
    MP_2_2ac,
    MP_2_2bc,
    MP_2ab_2,
    MP_2a_2ac,
    MP_2b_2c,
    MP_2a_2b,
    MP_2ac_2c,
    MP_2bc_2b,
    MP_2b_2ab,
    MP_2_2ab,
    MP_2bc_2,
    MP_2ac_2ac,
    MP_2ab_2ac,
    MP_2ac_2bc,
    MP_2bc_2ab,
    MP_2c_2b,
    MP_2c_2ac,
    MP_2ac_2a,
    MP_2b_2a,
    MP_2a_2ab,
    MP_2bc_2c,
    MP_2_2n,
    MP_2n_2,
    MP_2n_2n,
    P_2_2ab_M1ab,
    MP_2ab_2a,
    P_2bc_2_M1bc,
    MP_2c_2bc,
    P_2ac_2ac_M1ac,
    MP_2c_2a,
    MP_2n_2ab,
    MP_2n_2c,
    MP_2a_2n,
    MP_2bc_2n,
    MP_2ac_2b,
    MP_2b_2ac,
    MP_2ac_2ab,
    MP_2bc_2ac,
    MP_2ac_2n,
    MP_2bc_2a,
    MP_2c_2ab,
    MP_2n_2ac,
    MP_2n_2a,
    MP_2c_2n,
    MC_2c_2,
    MC_2c_2c,
    MA_2a_2a,
    MA_2_2a,
    MB_2_2b,
    MB_2b_2,
    MC_2bc_2,
    MC_2bc_2bc,
    MA_2ac_2ac,
    MA_2_2ac,
    MB_2_2bc,
    MB_2bc_2,
    MC_2_2,
    MA_2_2,
    MB_2_2,
    MC_2_2c,
    MA_2a_2,
    MB_2b_2b,
    MC_2b_2,
    MC_2b_2b,
    MA_2c_2c,
    MA_2_2c,
    MB_2_2c,
    MB_2c_2,
    C_2_2_M1bc_1,
    MC_2b_2bc,
    C_2_2_M1bc_2,
    MC_2b_2c,
    A_2_2_M1ac_1,
    MA_2a_2c,
    A_2_2_M1ac_2,
    MA_2ac_2c,
    B_2_2_M1bc_1,
    MB_2bc_2b,
    B_2_2_M1bc_2,
    MB_2b_2bc,
    MF_2_2,
    F_2_2_M1d,
    MF_2uv_2vw,
    MI_2_2,
    MI_2_2c,
    MI_2a_2,
    MI_2b_2b,
    MI_2b_2c,
    MI_2a_2b,
    MI_2b_2,
    MI_2a_2a,
    MI_2c_2c,
    MI_2_2b,
    MI_2_2a,
    MI_2c_2,
    P_4,
    P_4w,
    P_4c,
    P_4cw,
    I_4,
    I_4bw,
    P_M4,
    I_M4,
    MP_4,
    MP_4c,
    P_4ab_M1ab,
    MP_4a,
    P_4n_M1n,
    MP_4bc,
    MI_4,
    I_4bw_M1bw,
    MI_4ad,
    P_4_2,
    P_4ab_2ab,
    P_4w_2c,
    P_4abw_2nw,
    P_4c_2,
    P_4n_2n,
    P_4cw_2c,
    P_4nw_2abw,
    I_4_2,
    I_4bw_2bw,
    P_4_M2,
    P_4_M2ab,
    P_4c_M2c,
    P_4n_M2n,
    P_4_M2c,
    P_4_M2n,
    P_4c_M2,
    P_4c_M2ab,
    I_4_M2,
    I_4_M2c,
    I_4bw_M2,
    I_4bw_M2c,
    P_M4_2,
    P_M4_2c,
    P_M4_2ab,
    P_M4_2n,
    P_M4_M2,
    P_M4_M2c,
    P_M4_M2ab,
    P_M4_M2n,
    I_M4_M2,
    I_M4_M2c,
    I_M4_2,
    I_M4_2bw,
    MP_4_2,
    MP_4_2c,
    P_4_2_M1ab,
    MP_4a_2b,
    P_4_2_M1n,
    MP_4a_2bc,
    MP_4_2ab,
    MP_4_2n,
    P_4ab_2ab_M1ab,
    MP_4a_2a,
    P_4ab_2n_M1ab,
    MP_4a_2ac,
    MP_4c_2,
    MP_4c_2c,
    P_4n_2c_M1n,
    MP_4ac_2b,
    P_4n_2_M1n,
    MP_4ac_2bc,
    MP_4c_2ab,
    MP_4n_2n,
    P_4n_2n_M1n,
    MP_4ac_2a,
    P_4n_2ab_M1n,
    MP_4ac_2ac,
    MI_4_2,
    MI_4_2c,
    I_4bw_2bw_M1bw,
    MI_4bd_2,
    I_4bw_2aw_M1bw,
    MI_4bd_2c,
    P_3,
    P_31,
    P_32,
    R_3,
    P_3A,
    MP_3,
    MR_3,
    MP_3A,
    P_3_2,
    P_3_2D,
    P_31_2c_0_0_1,
    P_31_2D,
    P_32_2c_0_0_M1,
    P_32_2D,
    R_3_2D,
    P_3A_2,
    P_3_M2D,
    P_3_M2,
    P_3_M2Dc,
    P_3_M2c,
    R_3_M2D,
    P_3A_M2,
    R_3_M2Dc,
    P_3A_M2n,
    MP_3_2,
    MP_3_2c,
    MP_3_2D,
    MP_3_2Dc,
    MR_3_2D,
    MP_3A_2,
    MR_3_2Dc,
    MP_3A_2n,
    P_6,
    P_61,
    P_65,
    P_62,
    P_64,
    P_6c,
    P_M6,
    MP_6,
    MP_6c,
    P_6_2,
    P_61_2_0_0_M1,
    P_65_2_0_0_1,
    P_62_2c_0_0_1,
    P_64_2c_0_0_M1,
    P_6c_2c,
    P_6_M2,
    P_6_M2c,
    P_6c_M2,
    P_6c_M2c,
    P_M6_2,
    P_M6c_2,
    P_M6_M2,
    P_M6c_M2c,
    MP_6_2,
    MP_6_2c,
    MP_6c_2,
    MP_6c_2c,
    P_2_2_3,
    F_2_2_3,
    I_2_2_3,
    P_2ac_2ab_3,
    I_2b_2c_3,
    MP_2_2_3,
    P_2_2_3_M1n,
    MP_2ab_2bc_3,
    MF_2_2_3,
    F_2_2_3_M1d,
    MF_2uv_2vw_3,
    MI_2_2_3,
    MP_2ac_2ab_3,
    MI_2b_2c_3,
    P_4_2_3,
    P_4n_2_3,
    F_4_2_3,
    F_4d_2_3,
    I_4_2_3,
    P_4acd_2ab_3,
    P_4bd_2ab_3,
    I_4bd_2c_3,
    P_M4_2_3,
    F_M4_2_3,
    I_M4_2_3,
    P_M4n_2_3,
    F_M4c_2_3,
    I_M4bd_2c_3,
    MP_4_2_3,
    P_4_2_3_M1n,
    MP_4a_2bc_3,
    MP_4n_2_3,
    P_4n_2_3_M1n,
    MP_4bc_2bc_3,
    MF_4_2_3,
    MF_4c_2_3,
    F_4d_2_3_M1d,
    MF_4vw_2vw_3,
    F_4d_2_3_M1cd,
    MF_4cvw_2vw_3,
    MI_4_2_3,
    MI_4bd_2c_3,
}

impl SpaceGroupHallSymbol {
    pub fn get_hall_symbol(&self) -> String {
        let i = *self as usize;
        FULL_SPACE_GROUP_SYMBOLS
            .get_hall_symbol(i)
            .unwrap()
            .to_string()
    }
    pub fn get_hm_symbol(&self) -> String {
        let i = *self as usize;
        FULL_SPACE_GROUP_SYMBOLS
            .get_hm_full_notation(i)
            .unwrap()
            .to_string()
    }
    pub fn get_space_group_number_code(&self) -> String {
        let i = *self as usize;
        FULL_SPACE_GROUP_SYMBOLS
            .get_space_group_number(i)
            .unwrap()
            .to_string()
    }
}
