/*
    This file is part of rs3a.

    rs3a is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    Foobar is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with rs3a.  If not, see <https://www.gnu.org/licenses/>.
*/
mod tests {
    use crate::*;
    const A: &str = "	Header starts here
	Comments starts with tab char
width 22	Count of symbols in column
height 14	Cpunt of rows in frames
loop true
colors full	Colors are specified for both text and background
delay 200
@ In header comments also may starts with @ char



	There
	is
	one
	ore
	more
	empty
	lines
	between
	header
	and
	body



	Body starts here
	First frame
LU]Pk&3):F*k[]qbd;$0Bp77777777777777777777770000000000000000000000	First row
}YZ7Ik;=a^KF(0CxvF5AU+77777777777777777777770000000000000000000000	Second row
uPJZ]RJ3]^xsyJ~-;2~.dW777777cccccc77777777770000003333330000000000	Third row
pj&<rH.vFN6odJ5c-l~CRx777777cccccc77777777770000003333330000000000
A!ht(UB@(jNDl.67+n)?,N7777777777cc77cccccc770000000000330033333300
v6,gTX64AWFW>%>IdAxSgW7777777777cc77cccccc770000000000330033333300
}[FiPQzkcMdG@K!<@dvC-$777777cccccc77cc77cc770000003333330033003300
C:DT+Odt-P(0pu%r}vlr#H777777cccccc77cc77cc770000003333330033003300
wAD)<iMp>L}yh}Y+}-r$BN7777777777cc77cccccc770000000000330033333300
@,ekE:w[Xt8sCmxxFj9EHN7777777777cc77cccccc770000000000330033333300
rX7=W;uEZii2*xosZuDRBg77cc77cccccc77cc77cc770033003333330033003300
vHtkD411dJ>P~3=kkB4wU^77cc77cccccc77cc77cc770033003333330033003300
,6oFB+gD3f_%gJua{50=HD77777777777777777777770000000000000000000000
rq5?T0lL06Vg-[0G:,sum)77777777777777777777770000000000000000000000

	Second frame
lH<^&?@^Hbt^3v5]7gx9<o	First column in a first row
7777777777777777777777	Second column in a first row
0000000000000000000000	Third column in a first row
hoB<mg>DGKGfM3woWh4%EC77777777777777777777770000000000000000000000
Js%6!JG[m,C8PVsrG7_ESx777777cccccc77777777770000003333330000000000
dCX<E6]xxP~Hk(([D*dGU?777777cccccc77777777770000003333330000000000
~!gdx`w[f0z)xKbnj{rAWS7777777777cc77cccccc770000000000330033333300
5V=hf%`00gl2-YlT;L*&U`7777777777cc77cccccc770000000000330033333300
qk[3_hDMUNNIn}Y`F>lh@o777777cccccc77cc77cc770000003333330033003300
HsHD0=38OO.#iCA&~U[{kT777777cccccc77cc77cc77
0000003333330033003300v#x@h.OzV,zLI6#5&kv4T-777777777	You can add line breaks anywhere in the body.
7cc77cccccc770000000000330033333300			Any way they, like comments, are ignored by the parser.
#t:Gic$*w-i;P<O!fIWdC,7777777777cc77cccccc770000000000330033333300
zs{lk^a3Ty8SbYgLGe7Pdt77cc77cccccc77cc77cc770033003333330033003300
i#%5(w~u9+cdlOdO!&Ms}677cc77cccccc77cc77cc770033003333330033003300
Ou=JC8Zn.T;pq98cx)ov>~77777777777777777777770000000000000000000000
aNvuQi.U6%7-Kf,uk{FG[J77777777777777777777770000000000000000000000

	Third frame
Jk%bTX:]aZ,D?jkB0?*I*O77777777777777777777770000000000000000000000
r~&Em{~S%FVCv._]xdt5.,77777777777777777777770000000000000000000000
OEr=o[s:ocqCa;,h2a-d:q777777cccccc77777777770000003333330000000000
U6]~]fX>~%T%(RAa$p`~n!777777cccccc77777777770000003333330000000000
YRq`hu=gRKg.!k>82v#[^D7777777777cc77cccccc770000000000330033333300
C#F7Nq3U3!yopFgBSRoUPB7777777777cc77cccccc770000000000330033333300
)p_U9Fhxi]W1IE)=s$>dh^777777cccccc77cc77cc770000003333330033003300
8uhRPgAy2}uioTM5Rw@>*5777777cccccc77cc77cc770000003333330033003300
xgq#T=V5d}8WaQU+kez]>X7777777777cc77cccccc770000000000330033333300
ed`pS6%DK9N% iQo7-[gDO7777777777cc77cccccc770000000000330033333300
[Z:jGUY%L&$74[@Q8;Km~E77cc77cccccc77cc77cc770033003333330033003300
gQ{ikr-5fyM<{ny6=]r4U$77cc77cccccc77cc77cc770033003333330033003300
iVG.vOv5uWkulYY#GT[&Tm77777777777777777777770000000000000000000000
U0DC_D-@ml4[7sP7&)C9Q>77777777777777777777770000000000000000000000";
    const B: &str = "width 22
height 14
loop true
colors full
delay 200
@ In header comments also may starts with @ char






LU]Pk&3):F*k[]qbd;$0Bp77777777777777777777770000000000000000000000
}YZ7Ik;=a^KF(0CxvF5AU+77777777777777777777770000000000000000000000
uPJZ]RJ3]^xsyJ~-;2~.dW777777cccccc77777777770000003333330000000000
pj&<rH.vFN6odJ5c-l~CRx777777cccccc77777777770000003333330000000000
A!ht(UB@(jNDl.67+n)?,N7777777777cc77cccccc770000000000330033333300
v6,gTX64AWFW>%>IdAxSgW7777777777cc77cccccc770000000000330033333300
}[FiPQzkcMdG@K!<@dvC-$777777cccccc77cc77cc770000003333330033003300
C:DT+Odt-P(0pu%r}vlr#H777777cccccc77cc77cc770000003333330033003300
wAD)<iMp>L}yh}Y+}-r$BN7777777777cc77cccccc770000000000330033333300
@,ekE:w[Xt8sCmxxFj9EHN7777777777cc77cccccc770000000000330033333300
rX7=W;uEZii2*xosZuDRBg77cc77cccccc77cc77cc770033003333330033003300
vHtkD411dJ>P~3=kkB4wU^77cc77cccccc77cc77cc770033003333330033003300
,6oFB+gD3f_%gJua{50=HD77777777777777777777770000000000000000000000
rq5?T0lL06Vg-[0G:,sum)77777777777777777777770000000000000000000000

lH<^&?@^Hbt^3v5]7gx9<o
7777777777777777777777
0000000000000000000000
hoB<mg>DGKGfM3woWh4%EC77777777777777777777770000000000000000000000
Js%6!JG[m,C8PVsrG7_ESx777777cccccc77777777770000003333330000000000
dCX<E6]xxP~Hk(([D*dGU?777777cccccc77777777770000003333330000000000
~!gdx`w[f0z)xKbnj{rAWS7777777777cc77cccccc770000000000330033333300
5V=hf%`00gl2-YlT;L*&U`7777777777cc77cccccc770000000000330033333300
qk[3_hDMUNNIn}Y`F>lh@o777777cccccc77cc77cc770000003333330033003300
HsHD0=38OO.#iCA&~U[{kT777777cccccc77cc77cc77
0000003333330033003300v#x@h.OzV,zLI6#5&kv4T-777777777
7cc77cccccc770000000000330033333300
#t:Gic$*w-i;P<O!fIWdC,7777777777cc77cccccc770000000000330033333300
zs{lk^a3Ty8SbYgLGe7Pdt77cc77cccccc77cc77cc770033003333330033003300
i#%5(w~u9+cdlOdO!&Ms}677cc77cccccc77cc77cc770033003333330033003300
Ou=JC8Zn.T;pq98cx)ov>~77777777777777777777770000000000000000000000
aNvuQi.U6%7-Kf,uk{FG[J77777777777777777777770000000000000000000000

Jk%bTX:]aZ,D?jkB0?*I*O77777777777777777777770000000000000000000000
r~&Em{~S%FVCv._]xdt5.,77777777777777777777770000000000000000000000
OEr=o[s:ocqCa;,h2a-d:q777777cccccc77777777770000003333330000000000
U6]~]fX>~%T%(RAa$p`~n!777777cccccc77777777770000003333330000000000
YRq`hu=gRKg.!k>82v#[^D7777777777cc77cccccc770000000000330033333300
C#F7Nq3U3!yopFgBSRoUPB7777777777cc77cccccc770000000000330033333300
)p_U9Fhxi]W1IE)=s$>dh^777777cccccc77cc77cc770000003333330033003300
8uhRPgAy2}uioTM5Rw@>*5777777cccccc77cc77cc770000003333330033003300
xgq#T=V5d}8WaQU+kez]>X7777777777cc77cccccc770000000000330033333300
ed`pS6%DK9N% iQo7-[gDO7777777777cc77cccccc770000000000330033333300
[Z:jGUY%L&$74[@Q8;Km~E77cc77cccccc77cc77cc770033003333330033003300
gQ{ikr-5fyM<{ny6=]r4U$77cc77cccccc77cc77cc770033003333330033003300
iVG.vOv5uWkulYY#GT[&Tm77777777777777777777770000000000000000000000
U0DC_D-@ml4[7sP7&)C9Q>77777777777777777777770000000000000000000000";
    #[test]
    fn test_escape_comments(){
        assert_eq!(escape_comments(A), B.to_string());
    }
}

mod body_to_text_test{
    use crate::*;
    #[test]
    fn test_correct_fullcolor(){
        let text_reference = "AAAAaabb1122\nBBBBaabc1122\nCCCCaaaa1111\nDDDDabcd1111\n\nAAAAaabb1122\nBBBBaabc1122\nCCCCaaaa1111\nDDDDabcd1111\n";
        let body = Body{
            frames: vec![
                vec![
                    vec![
                        RowFragment{
                            text: "AA".to_string(),
                            bg_color: Some(Color::BLUE),
                            fg_color: Some(Color::BRIGHT_GREEN),
                        },
                        RowFragment{
                            text: "AA".to_string(),
                            bg_color: Some(Color::GREEN),
                            fg_color: Some(Color::BRIGHT_CYAN),
                        },
                    ],
                    vec![
                        RowFragment{
                            text: "BB".to_string(),
                            bg_color: Some(Color::BLUE),
                            fg_color: Some(Color::BRIGHT_GREEN),
                        },
                        RowFragment{
                            text: "B".to_string(),
                            bg_color: Some(Color::GREEN),
                            fg_color: Some(Color::BRIGHT_CYAN),
                        },
                        RowFragment{
                            text: "B".to_string(),
                            bg_color: Some(Color::GREEN),
                            fg_color: Some(Color::BRIGHT_RED),
                        },
                    ],
                    vec![
                        RowFragment{
                            text: "CCCC".to_string(),
                            bg_color: Some(Color::BLUE),
                            fg_color: Some(Color::BRIGHT_GREEN),
                        },
                    ],
                    vec![
                        RowFragment{
                            text: "D".to_string(),
                            bg_color: Some(Color::BLUE),
                            fg_color: Some(Color::BRIGHT_GREEN),
                        },
                        RowFragment{
                            text: "D".to_string(),
                            bg_color: Some(Color::BLUE),
                            fg_color: Some(Color::BRIGHT_CYAN),
                        },
                        RowFragment{
                            text: "D".to_string(),
                            bg_color: Some(Color::BLUE),
                            fg_color: Some(Color::BRIGHT_RED),
                        },
                        RowFragment{
                            text: "D".to_string(),
                            bg_color: Some(Color::BLUE),
                            fg_color: Some(Color::BRIGHT_MAGENTA),
                        },
                    ],
                ],
                vec![
                    vec![
                        RowFragment{
                            text: "AA".to_string(),
                            bg_color: Some(Color::BLUE),
                            fg_color: Some(Color::BRIGHT_GREEN),
                        },
                        RowFragment{
                            text: "AA".to_string(),
                            bg_color: Some(Color::GREEN),
                            fg_color: Some(Color::BRIGHT_CYAN),
                        },
                    ],
                    vec![
                        RowFragment{
                            text: "BB".to_string(),
                            bg_color: Some(Color::BLUE),
                            fg_color: Some(Color::BRIGHT_GREEN),
                        },
                        RowFragment{
                            text: "B".to_string(),
                            bg_color: Some(Color::GREEN),
                            fg_color: Some(Color::BRIGHT_CYAN),
                        },
                        RowFragment{
                            text: "B".to_string(),
                            bg_color: Some(Color::GREEN),
                            fg_color: Some(Color::BRIGHT_RED),
                        },
                    ],
                    vec![
                        RowFragment{
                            text: "CCCC".to_string(),
                            bg_color: Some(Color::BLUE),
                            fg_color: Some(Color::BRIGHT_GREEN),
                        },
                    ],
                    vec![
                        RowFragment{
                            text: "D".to_string(),
                            bg_color: Some(Color::BLUE),
                            fg_color: Some(Color::BRIGHT_GREEN),
                        },
                        RowFragment{
                            text: "D".to_string(),
                            bg_color: Some(Color::BLUE),
                            fg_color: Some(Color::BRIGHT_CYAN),
                        },
                        RowFragment{
                            text: "D".to_string(),
                            bg_color: Some(Color::BLUE),
                            fg_color: Some(Color::BRIGHT_RED),
                        },
                        RowFragment{
                            text: "D".to_string(),
                            bg_color: Some(Color::BLUE),
                            fg_color: Some(Color::BRIGHT_MAGENTA),
                        },
                    ],
                ],
            ],
        };
        assert_eq!(body.to_string(true), text_reference);
    }
}

mod body_from_text_test{
    use crate::*;
    #[test]
    fn test_correct_fullcolor(){
        let header = Header{
            width: 4,
            height: 4,
            delay: 200,
            loop_enable: true,
            color_mod: ColorMod::Full,
            utf8: false,
            datacols: 3,
            preview: DEFAULT_PREVIEW,
            audio: None,
            title: None,
            author: None,
        };
        let body_reference = Body{
            frames: vec![
                vec![
                    vec![
                        RowFragment{
                            text: "AA".to_string(),
                            bg_color: Some(Color::BLUE),
                            fg_color: Some(Color::BRIGHT_GREEN),
                        },
                        RowFragment{
                            text: "AA".to_string(),
                            bg_color: Some(Color::GREEN),
                            fg_color: Some(Color::BRIGHT_CYAN),
                        },
                    ],
                    vec![
                        RowFragment{
                            text: "BB".to_string(),
                            bg_color: Some(Color::BLUE),
                            fg_color: Some(Color::BRIGHT_GREEN),
                        },
                        RowFragment{
                            text: "B".to_string(),
                            bg_color: Some(Color::GREEN),
                            fg_color: Some(Color::BRIGHT_CYAN),
                        },
                        RowFragment{
                            text: "B".to_string(),
                            bg_color: Some(Color::GREEN),
                            fg_color: Some(Color::BRIGHT_RED),
                        },
                    ],
                    vec![
                        RowFragment{
                            text: "CCCC".to_string(),
                            bg_color: Some(Color::BLUE),
                            fg_color: Some(Color::BRIGHT_GREEN),
                        },
                    ],
                    vec![
                        RowFragment{
                            text: "D".to_string(),
                            bg_color: Some(Color::BLUE),
                            fg_color: Some(Color::BRIGHT_GREEN),
                        },
                        RowFragment{
                            text: "D".to_string(),
                            bg_color: Some(Color::BLUE),
                            fg_color: Some(Color::BRIGHT_CYAN),
                        },
                        RowFragment{
                            text: "D".to_string(),
                            bg_color: Some(Color::BLUE),
                            fg_color: Some(Color::BRIGHT_RED),
                        },
                        RowFragment{
                            text: "D".to_string(),
                            bg_color: Some(Color::BLUE),
                            fg_color: Some(Color::BRIGHT_MAGENTA),
                        },
                    ],
                ],
                vec![
                    vec![
                        RowFragment{
                            text: "AA".to_string(),
                            bg_color: Some(Color::BLUE),
                            fg_color: Some(Color::BRIGHT_GREEN),
                        },
                        RowFragment{
                            text: "AA".to_string(),
                            bg_color: Some(Color::GREEN),
                            fg_color: Some(Color::BRIGHT_CYAN),
                        },
                    ],
                    vec![
                        RowFragment{
                            text: "BB".to_string(),
                            bg_color: Some(Color::BLUE),
                            fg_color: Some(Color::BRIGHT_GREEN),
                        },
                        RowFragment{
                            text: "B".to_string(),
                            bg_color: Some(Color::GREEN),
                            fg_color: Some(Color::BRIGHT_CYAN),
                        },
                        RowFragment{
                            text: "B".to_string(),
                            bg_color: Some(Color::GREEN),
                            fg_color: Some(Color::BRIGHT_RED),
                        },
                    ],
                    vec![
                        RowFragment{
                            text: "CCCC".to_string(),
                            bg_color: Some(Color::BLUE),
                            fg_color: Some(Color::BRIGHT_GREEN),
                        },
                    ],
                    vec![
                        RowFragment{
                            text: "D".to_string(),
                            bg_color: Some(Color::BLUE),
                            fg_color: Some(Color::BRIGHT_GREEN),
                        },
                        RowFragment{
                            text: "D".to_string(),
                            bg_color: Some(Color::BLUE),
                            fg_color: Some(Color::BRIGHT_CYAN),
                        },
                        RowFragment{
                            text: "D".to_string(),
                            bg_color: Some(Color::BLUE),
                            fg_color: Some(Color::BRIGHT_RED),
                        },
                        RowFragment{
                            text: "D".to_string(),
                            bg_color: Some(Color::BLUE),
                            fg_color: Some(Color::BRIGHT_MAGENTA),
                        },
                    ],
                ],
            ],
        };
        let text = "AAAAaabb1122\nBBBBaabc1122\nCCCCaaaa1111\nDDDDabcd1111\n\nAAAAaabb1122\nBBBBaabc1122\nCCCCaaaa1111\nDDDDabcd1111\n";
        match Body::from_string(text.to_string(),header) {
            Ok(result) => {assert_eq!(body_reference, result);}
            Err(_) => {assert!(false, "Unexpected parcing error");}
        }
    }
}

mod colormod_test{
    use crate::*;
    use std::convert::{TryFrom, Into};
    #[test]
    fn test_none(){
        let base = ColorMod::None;
        let s: String = base.into();
        let s: &str = &s;
        match ColorMod::try_from(s){
            Ok(result) => {assert_eq!(base, result);}
            Err(_) => {assert!(false, "Unexpected parcing error");}
        }
    }
    #[test]
    fn test_fg(){
        let base = ColorMod::Fg;
        let s: String = base.into();
        let s: &str = &s;
        match ColorMod::try_from(s){
            Ok(result) => {assert_eq!(base, result);}
            Err(_) => {assert!(false, "Unexpected parcing error");}
        }
    }
    #[test]
    fn test_bg(){
        let base = ColorMod::Bg;
        let s: String = base.into();
        let s: &str = &s;
        match ColorMod::try_from(s){
            Ok(result) => {assert_eq!(base, result);}
            Err(_) => {assert!(false, "Unexpected parcing error");}
        }
    }
    #[test]
    fn test_full(){
        let base = ColorMod::Full;
        let s: String = base.into();
        let s: &str = &s;
        match ColorMod::try_from(s){
            Ok(result) => {assert_eq!(base, result);}
            Err(_) => {assert!(false, "Unexpected parcing error");}
        }
    }
}

mod header_to_string_tests{
    use crate::*;
    #[test]
    fn all_params(){
        let header = Header{
            width: 1,
            height: 2,
            delay: DEFAULT_DELAY+1,
            loop_enable: !DEFAULT_LOOP,
            color_mod: ColorMod::Full,
            utf8: true,
            datacols: 123,
            preview: 1,
            audio: Some("1234567".to_string()),
            title: None,
            author: None,
        };
        let s_ref = "width 1\nheight 2\ndelay 51\nloop false\ncolors full\nutf8\ndatacols 123\npreview 1\naudio 1234567\n\n".to_string();
        let s: String = header.into();
        assert_eq!(s_ref, s);
    }
    #[test]
    fn default_params(){
        let header = Header{
            width: 1,
            height: 2,
            delay: DEFAULT_DELAY,
            loop_enable: DEFAULT_LOOP,
            color_mod: DEFAULT_COLORS,
            utf8: DEFAULT_UTF8,
            datacols: DEFAULT_COLORS.to_datacols(),
            preview: DEFAULT_PREVIEW,
            audio: None,
            title: None,
            author: None,
        };
        let s_ref = "width 1\nheight 2\n\n".to_string();
        let s: String = header.into();
        assert_eq!(s_ref, s);
    }
    #[test]
    fn datacols(){
        let header = Header{
            width: 1,
            height: 2,
            delay: DEFAULT_DELAY,
            loop_enable: DEFAULT_LOOP,
            color_mod: DEFAULT_COLORS,
            utf8: DEFAULT_UTF8,
            datacols: DEFAULT_COLORS.to_datacols()+1,
            preview: DEFAULT_PREVIEW,
            audio: None,
            title: None,
            author: None,
        };
        let s_ref = "width 1\nheight 2\ndatacols 2\n\n".to_string();
        let s: String = header.into();
        assert_eq!(s_ref, s);
    }
}

mod header_from_string_tests{
    use crate::*;
    use std::convert::{TryFrom};
    #[test]
    fn full(){
        let s = "width 1\nheight 2\ndelay 3\nloop false\ncolors full\nutf8\ndatacols 5\npreview 1\naudio 12345".to_string();
        let refernce = Header{
            width: 1,
            height: 2,
            delay: 3,
            loop_enable: false,
            color_mod: ColorMod::Full,
            utf8: true,
            datacols: 5,
            preview: 1,
            audio: Some("12345".to_string()),
            title: None,
            author: None,
        };
        match Header::try_from(s){
            Ok(result) => { assert_eq!(refernce, result); }
            Err(_) => { assert!(false, "Unexpected parcing error"); }
        }
    }
    #[test]
    fn only_required(){
        let s = "width 1\nheight 2".to_string();
        let refernce = Header{
            width: 1,
            height: 2,
             delay: 50,
            loop_enable: true,
            color_mod: ColorMod::None,
            utf8: false,
            datacols: 1,
            preview: 0,
            audio: None,
            title: None,
            author: None,
        };
        match Header::try_from(s){
            Ok(result) => { assert_eq!(refernce, result); }
            Err(_) => { assert!(false, "Unexpected parcing error"); }
        }
    }
    #[test]
    fn optional_incorrect(){
        let s = "width 1\nheight 2\ndelay safdsfsdf\nloop dsfsdf\ncolors dfdfdf\ndatacols dfsfsddf".to_string();
        let refernce = Header{
            width: 1,
            height: 2,
             delay: 50,
            loop_enable: true,
            color_mod: ColorMod::None,
            utf8: false,
            datacols: 1,
            preview: 0,
            audio: None,
            title: None,
            author: None,
        };
        match Header::try_from(s){
            Ok(result) => { assert_eq!(refernce, result); }
            Err(_) => { assert!(false, "Unexpected parcing error"); }
        }
    }
    #[test]
    fn width_incorrect(){
        let s = "width sdfsfsdf\nheight 2\ndelay 3\nloop false\ncolors full\nutf8\ndatacols 5\naudio 12345".to_string();
        if let Ok(_) = Header::try_from(s){
            assert!(false, "Unexpected Ok");
        }
    }
    #[test]
    fn height_incorrect(){
        let s = "width 1\nheight sdfsdfsdf\ndelay 3\nloop false\ncolors full\nutf8\ndatacols 5\naudio 12345".to_string();
        if let Ok(_) = Header::try_from(s){
            assert!(false, "Unexpected Ok");
        }
    }
    #[test]
    fn datacols(){
        let s = "width 1\nheight 2\ncolors full".to_string();
        let refernce = Header{
            width: 1,
            height: 2,
             delay: 50,
            loop_enable: true,
            color_mod: ColorMod::Full,
            utf8: false,
            datacols: 3,
            preview: 0,
            audio: None,
            title: None,
            author: None,
        };
        match Header::try_from(s){
            Ok(result) => { assert_eq!(refernce, result); }
            Err(_) => { assert!(false, "Unexpected parcing error"); }
        }
        let s = "width 1\nheight 2\ncolors full\ndatacols 0".to_string();
        let refernce = Header{
            width: 1,
            height: 2,
             delay: 50,
            loop_enable: true,
            color_mod: ColorMod::Full,
            utf8: false,
            datacols: 0,
            preview: 0,
            audio: None,
            title: None,
            author: None,
        };
        match Header::try_from(s){
            Ok(result) => { assert_eq!(refernce, result); }
            Err(_) => { assert!(false, "Unexpected parcing error"); }
        }
    }
    #[test]
    fn extra_spaces(){
        let s = "width    1\nheight    2\ndelay    3\nloop    false\ncolors    full \
        \nutf8   \ndatacols    5\naudio    12345".to_string();
        let refernce = Header{
            width: 1,
            height: 2,
            delay: 3,
            loop_enable: false,
            color_mod: ColorMod::Full,
            utf8: true,
            datacols: 5,
            preview: 0,
            audio: Some("12345".to_string()),
            title: None,
            author: None,
        };
        match Header::try_from(s){
            Ok(result) => { assert_eq!(refernce, result); }
            Err(_) => { assert!(false, "Unexpected parcing error"); }
        }
    }
    #[test]
    fn extra_params(){
        let s = "width 1   sfdfsdf fdsfd sdf \nheight 2 fds dsfsdf\ndelay 3 fd ff \
        \nloop false   fdfdf  \ncolors full fdfd\nutf8  fdfdf\ndatacols 5 fdfd fd d\naudio 12345 fdfdfdf".to_string();
        let refernce = Header{
            width: 1,
            height: 2,
            delay: 3,
            loop_enable: false,
            color_mod: ColorMod::Full,
            utf8: true,
            datacols: 5,
            preview: 0,
            audio: Some("12345".to_string()),
            title: None,
            author: None,
        };
        match Header::try_from(s){
            Ok(result) => { assert_eq!(refernce, result); }
            Err(_) => { assert!(false, "Unexpected parcing error"); }
        }
    }
}

