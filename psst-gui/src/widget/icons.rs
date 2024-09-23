use crate::ui::theme;
use druid::{kurbo::BezPath, widget::prelude::*, Affine, Color, KeyOrValue, Size};

#[allow(dead_code)]
pub static LOGO: SvgIcon = SvgIcon {
    svg_path: "M2.34146 0.685791L28.5825 26.9268L26.9268 28.5825L0.685791 2.34145L2.34146 0.685791Z M22.2439 11.7073V15.2195C22.2441 15.8464 22.1721 16.4712 22.0295 17.0817L23.9012 18.9512C24.3547 17.7594 24.5865 16.4947 24.5854 15.2195V11.7073H22.2439ZM15.2195 29.2683V25.691C16.6827 25.5282 18.0952 25.0587 19.3646 24.3132L17.6342 22.5841C16.3849 23.1891 15.0025 23.4672 13.6165 23.3925C12.2305 23.3178 10.886 22.8926 9.70906 22.1568C8.53207 21.421 7.56101 20.3986 6.88673 19.1853C6.21245 17.9721 5.85701 16.6076 5.85367 15.2195V11.7073H3.51221V15.2195C3.51221 20.6341 7.61708 25.1063 12.8781 25.691V29.2683H8.19513V31.6098H19.9025V29.2683H15.2195ZM19.9025 14.9539V7.02438C19.9025 3.74194 17.3312 1.17072 14.0488 1.17072C13.029 1.16706 12.0261 1.43096 11.1402 1.93606C10.2542 2.44117 9.5163 3.16981 9.00001 4.04926 M8.19513 13.1437V15.1464C8.19618 16.706 8.81282 18.2022 9.91098 19.3098C10.66 20.0884 11.6134 20.6402 12.6617 20.9017C13.71 21.1633 14.8108 21.1241 15.8378 20.7886L8.19513 13.1437Z",
    svg_size: Size::new(29.0, 32.0),
    op: PaintOp::Fill,
};

// SF Pro Regular - gearshape
pub static PREFERENCES: SvgIcon = SvgIcon {
    svg_path: "M13.1035 23.208H14.8877C15.6172 23.208 16.1885 22.751 16.3643 22.0479L16.7158 20.5098L16.9443 20.4219L18.2891 21.2568C18.9043 21.6436 19.6338 21.5381 20.1523 21.0195L21.3828 19.7891C21.9102 19.2617 21.998 18.541 21.6113 17.9346L20.7764 16.5898L20.8643 16.3789L22.4023 16.0186C23.0967 15.8428 23.5537 15.2715 23.5537 14.542V12.8105C23.5537 12.0811 23.1055 11.5098 22.4023 11.334L20.873 10.9648L20.7852 10.7363L21.6201 9.40039C22.0068 8.79395 21.9189 8.07324 21.3916 7.53711L20.1611 6.30664C19.6514 5.78809 18.9219 5.69141 18.3066 6.06934L16.9619 6.89551L16.7158 6.80762L16.3643 5.26074C16.1885 4.55762 15.6172 4.10938 14.8877 4.10938H13.1035C12.3652 4.10938 11.7939 4.55762 11.627 5.26074L11.2754 6.80762L11.0293 6.89551L9.68457 6.06934C9.06055 5.69141 8.33984 5.78809 7.83008 6.30664L6.59082 7.53711C6.07227 8.07324 5.97559 8.79395 6.3623 9.40039L7.19727 10.7363L7.10938 10.9648L5.58887 11.334C4.88574 11.5098 4.4375 12.0811 4.4375 12.8105V14.542C4.4375 15.2715 4.89453 15.8428 5.58887 16.0186L7.12695 16.3789L7.20605 16.5898L6.37109 17.9346C5.98438 18.541 6.08105 19.2617 6.59961 19.7891L7.83887 21.0195C8.34863 21.5381 9.07812 21.6436 9.69336 21.2568L11.0381 20.4219L11.2754 20.5098L11.627 22.0479C11.7939 22.751 12.3652 23.208 13.1035 23.208ZM13.332 21.5908C13.1826 21.5908 13.1035 21.5293 13.0859 21.3975L12.5586 19.2354C12.0049 19.1035 11.4688 18.875 11.0381 18.6025L9.13965 19.7715C9.02539 19.8418 8.91992 19.833 8.81445 19.7275L7.8916 18.8047C7.78613 18.708 7.78613 18.6025 7.85645 18.4883L9.02539 16.5898C8.7793 16.168 8.55078 15.6406 8.41895 15.0869L6.24805 14.5684C6.11621 14.5508 6.0459 14.4717 6.0459 14.3223V13.0215C6.0459 12.8633 6.10742 12.8018 6.24805 12.7666L8.41016 12.2568C8.54199 11.668 8.79688 11.123 9.0166 10.7275L7.84766 8.84668C7.77734 8.72363 7.77734 8.61816 7.87402 8.5127L8.80566 7.59863C8.91113 7.50195 9.00781 7.48438 9.13965 7.56348L11.0205 8.71484C11.416 8.46875 12.0049 8.22266 12.5674 8.08203L13.0859 5.91992C13.1035 5.78809 13.1826 5.71777 13.332 5.71777H14.6592C14.8086 5.71777 14.8789 5.7793 14.9053 5.91992L15.4326 8.09082C16.0039 8.23145 16.5225 8.46875 16.9619 8.71484L18.8428 7.56348C18.9746 7.49316 19.0713 7.50195 19.1768 7.60742L20.1084 8.52148C20.2139 8.61816 20.2139 8.72363 20.1348 8.84668L18.9746 10.7275C19.1855 11.123 19.4492 11.668 19.5811 12.2568L21.7432 12.7666C21.8838 12.8018 21.9365 12.8633 21.9365 13.0215V14.3223C21.9365 14.4717 21.875 14.5508 21.7432 14.5684L19.5723 15.0869C19.4404 15.6406 19.2031 16.1768 18.957 16.5898L20.126 18.4795C20.1963 18.6025 20.1963 18.6992 20.0908 18.7959L19.168 19.7275C19.0625 19.833 18.957 19.8418 18.8428 19.7715L16.9531 18.6025C16.5137 18.875 16.0127 19.0947 15.4326 19.2354L14.9053 21.3975C14.8789 21.5293 14.8086 21.5908 14.6592 21.5908H13.332ZM14 16.9941C15.8281 16.9941 17.3311 15.4912 17.3311 13.6543C17.3311 11.835 15.8281 10.332 14 10.332C12.1631 10.332 10.6514 11.835 10.6514 13.6543C10.6514 15.4912 12.1631 16.9941 14 16.9941ZM14 15.4736C12.998 15.4736 12.1807 14.6562 12.1807 13.6543C12.1807 12.6699 13.0068 11.8525 14 11.8525C14.9756 11.8525 15.793 12.6699 15.793 13.6543C15.793 14.6475 14.9756 15.4736 14 15.4736Z",
    svg_size: Size::new(28.0, 28.0),
    op: PaintOp::Fill,
};
// SF Pro Regular - key
pub static ACCOUNT: SvgIcon = SvgIcon {
    svg_path: "M13.332 24.3682C13.7363 24.7197 14.29 24.7637 14.6768 24.377L17.3926 21.6611C17.7705 21.2832 17.7529 20.6943 17.3838 20.3164L16.0918 19.0244L18.0078 17.1084C18.377 16.7393 18.377 16.1416 17.999 15.7637L16.25 14.0059C18.6318 12.8193 19.9678 10.8418 19.9678 8.5127C19.9678 5.2168 17.3047 2.55371 14 2.55371C10.6865 2.55371 8.03223 5.20801 8.03223 8.5127C8.03223 10.877 9.37695 12.9951 11.5215 13.9619V22.2061C11.5215 22.5225 11.6182 22.8828 11.8906 23.1201L13.332 24.3682ZM14 22.8037L13.0508 21.8545V12.8018C11.0469 12.3623 9.61426 10.6045 9.61426 8.5127C9.61426 6.0957 11.5654 4.14453 14 4.14453C16.4346 4.14453 18.377 6.0957 18.377 8.5127C18.377 10.5869 16.9355 12.3711 14.7383 12.8545V14.7617L16.4258 16.4492L14.624 18.2158V19.8066L15.8105 20.9756L14 22.8037ZM14 8.56543C14.8613 8.56543 15.5645 7.8623 15.5645 7.00098C15.5645 6.13965 14.8613 5.43652 14 5.43652C13.1299 5.43652 12.4355 6.13086 12.4355 7.00098C12.4355 7.8623 13.1387 8.56543 14 8.56543Z",
    svg_size: Size::new(28.0, 28.0),
    op: PaintOp::Fill,
};
// SF Pro Regular - internaldrive
pub static STORAGE: SvgIcon = SvgIcon {
    svg_path: "M3.40039 17.2139C3.40039 19.5781 5.17578 21.3535 7.71582 21.3535H20.2842C22.8242 21.3535 24.5996 19.5781 24.5996 17.2139C24.5996 16.502 24.3975 15.8604 24.1514 15.2803L21.1279 8.19629C20.5127 6.74609 19.291 5.98145 17.6562 5.98145H10.3525C8.70898 5.98145 7.4873 6.74609 6.88086 8.19629L3.875 15.2451C3.62012 15.834 3.40039 16.4844 3.40039 17.2139ZM6.57324 13.2676L8.48926 8.5918C8.78809 7.83594 9.46484 7.44043 10.3701 7.44043H17.6299C18.5439 7.44043 19.2207 7.83594 19.5195 8.5918L21.4355 13.2676C21.084 13.1533 20.6973 13.083 20.2842 13.083H7.71582C7.30273 13.083 6.9248 13.1533 6.57324 13.2676ZM5.08789 17.2139C5.08789 15.8164 6.13379 14.7705 7.71582 14.7705H20.2842C21.8662 14.7705 22.9121 15.8164 22.9121 17.2139C22.9121 18.7607 21.8662 19.6572 20.2842 19.6572H7.71582C6.13379 19.6572 5.08789 18.6201 5.08789 17.2139ZM11.4512 18.084C11.4512 18.3828 11.6885 18.6113 11.9873 18.6113C12.2773 18.6113 12.5059 18.3828 12.5059 18.084V16.3525C12.5059 16.0625 12.2773 15.8252 11.9873 15.8252C11.6885 15.8252 11.4512 16.0625 11.4512 16.3525V18.084ZM13.4727 18.084C13.4727 18.3828 13.7012 18.6113 14 18.6113C14.29 18.6113 14.5273 18.3828 14.5273 18.084V16.3525C14.5273 16.0625 14.29 15.8252 14 15.8252C13.7012 15.8252 13.4727 16.0625 13.4727 16.3525V18.084ZM15.4854 18.084C15.4854 18.3828 15.7227 18.6113 16.0215 18.6113C16.3115 18.6113 16.5488 18.3828 16.5488 18.084V16.3525C16.5488 16.0625 16.3115 15.8252 16.0215 15.8252C15.7227 15.8252 15.4854 16.0625 15.4854 16.3525V18.084ZM17.5068 18.084C17.5068 18.3828 17.7441 18.6113 18.043 18.6113C18.333 18.6113 18.5703 18.3828 18.5703 18.084V16.3525C18.5703 16.0625 18.333 15.8252 18.043 15.8252C17.7441 15.8252 17.5068 16.0625 17.5068 16.3525V18.084ZM19.5283 18.084C19.5283 18.3828 19.7656 18.6113 20.0645 18.6113C20.3545 18.6113 20.583 18.3828 20.583 18.084V16.3525C20.583 16.0625 20.3545 15.8252 20.0645 15.8252C19.7656 15.8252 19.5283 16.0625 19.5283 16.3525V18.084Z",
    svg_size: Size::new(28.0, 28.0),
    op: PaintOp::Fill,
};

pub static BACK: SvgIcon = SvgIcon {
    svg_path: "M9.70711 0.292893C10.0976 0.683417 10.0976 1.31658 9.70711 1.70711L2.41421 9L9.70711 16.2929C10.0976 16.6834 10.0976 17.3166 9.70711 17.7071C9.31658 18.0976 8.68342 18.0976 8.29289 17.7071L0.292893 9.70711C-0.0976311 9.31658 -0.0976311 8.68342 0.292893 8.29289L8.29289 0.292893C8.68342 -0.0976311 9.31658 -0.0976311 9.70711 0.292893Z",
    svg_size: Size::new(10.0, 18.0),
    op: PaintOp::Fill,
};

pub static DOWN: SvgIcon = SvgIcon {
    svg_path: "m -3.7071056,4.292866 c 0.390524,-0.39049 1.023687,-0.39049 1.414217,0 l 7.2928898,7.2929 7.2928998,-7.2929 c 0.3905,-0.39049 1.0237,-0.39049 1.4142,0 0.3905,0.39053 0.3905,1.02369 0,1.41422 l -7.9999898,7.999997 c -0.39053,0.390525 -1.02369,0.390525 -1.41422,0 L -3.7071056,5.707086 c -0.390524,-0.39053 -0.390524,-1.02369 0,-1.41422 z",
    svg_size: Size::new(10.0, 18.0),
    op: PaintOp::Fill,
};

pub static UP: SvgIcon = SvgIcon {
svg_path: "m 13.707083,13.707109 c -0.390524,0.39049 -1.023687,0.39049 -1.414217,0 L 4.9999761,6.4142094 -2.2929238,13.707109 c -0.3905,0.39049 -1.0237,0.39049 -1.4142,0 -0.3905,-0.39053 -0.3905,-1.02369 0,-1.41422 L 4.2928661,4.2928924 c 0.39053,-0.390525 1.02369,-0.390525 1.41422,0 l 7.9999969,7.9999966 c 0.390524,0.39053 0.390524,1.02369 0,1.41422 z",
svg_size: Size::new(10.0, 18.0),
op: PaintOp::Fill,
};

// SF Pro Regular - play.fill
pub static PLAY: SvgIcon = SvgIcon {
    svg_path: "M9.3418 21.3711C9.71094 21.3711 10.0361 21.2393 10.4404 21.002L20.8203 14.999C21.5762 14.5596 21.8926 14.2168 21.8926 13.6631C21.8926 13.1094 21.5762 12.7754 20.8203 12.3271L10.4404 6.32422C10.0361 6.08691 9.71094 5.95508 9.3418 5.95508C8.62109 5.95508 8.11133 6.50879 8.11133 7.37891V19.9473C8.11133 20.8262 8.62109 21.3711 9.3418 21.3711Z",
    svg_size: Size::new(28.0, 28.0),
    op: PaintOp::Fill,
};
// SF Pro Regular - pause.fill
pub static PAUSE: SvgIcon = SvgIcon {
    svg_path: "M9.63184 20.9668H11.6973C12.5322 20.9668 12.9629 20.5361 12.9629 19.7012V7.60742C12.9629 6.75488 12.5322 6.35059 11.6973 6.3418H9.63184C8.79688 6.3418 8.36621 6.77246 8.36621 7.60742V19.7012C8.35742 20.5361 8.78809 20.9668 9.63184 20.9668ZM16.3115 20.9668H18.3682C19.2031 20.9668 19.6338 20.5361 19.6338 19.7012V7.60742C19.6338 6.75488 19.2031 6.3418 18.3682 6.3418H16.3115C15.4678 6.3418 15.0459 6.77246 15.0459 7.60742V19.7012C15.0459 20.5361 15.4678 20.9668 16.3115 20.9668Z",
    svg_size: Size::new(28.0, 28.0),
    op: PaintOp::Fill,
};
// SF Pro Regular - backward.end.alt.fill
pub static SKIP_BACK: SvgIcon = SvgIcon {
    svg_path: "M -0.449872 21.1701 L 1.63705 21.1701 C 2.43338 21.1701 2.85442 20.7491 2.85442 19.9436 L 2.85442 14.0581 C 2.95511 14.4425 3.23885 14.772 3.73313 15.0741 L 13.0602 20.5569 C 13.4355 20.7765 13.7375 20.8955 14.1128 20.8955 C 14.8359 20.8955 15.4309 20.3555 15.4309 19.3303 L 15.4309 14.0581 C 15.5316 14.4425 15.8153 14.772 16.3096 15.0741 L 25.6367 20.5569 C 26.012 20.7765 26.314 20.8955 26.6893 20.8955 C 27.4124 20.8955 28.0074 20.3555 28.0074 19.3303 L 28.0073 8.09025 C 28.0073 7.06509 27.4123 6.5159 26.6892 6.5159 C 26.314 6.5159 26.0119 6.63489 25.6366 6.85457 L 16.3096 12.3465 C 15.8153 12.6394 15.5316 12.9781 15.4309 13.3534 L 15.4309 8.09025 C 15.4309 7.06509 14.8359 6.5159 14.1128 6.5159 C 13.7376 6.5159 13.4355 6.63489 13.0602 6.85457 L 3.73313 12.3465 C 3.23886 12.6394 2.95511 12.978 2.85442 13.3533 L 2.85442 7.47699 C 2.85442 6.64405 2.43337 6.25962 1.63705 6.25962 L -0.449872 6.25962 C -1.2462 6.25962 -1.66724 6.68067 -1.66724 7.47699 L -1.66724 19.9436 C -1.66724 20.7491 -1.24619 21.1701 -0.44987 21.1701 Z",
    svg_size: Size::new(28.0, 28.0),
    op: PaintOp::Fill,
};
// SF Pro Regular - forward.end.alt.fill
pub static SKIP_FORWARD: SvgIcon = SvgIcon {
    svg_path: "M 0.909093 20.6922 C 1.28439 20.6922 1.58649 20.5732 1.96179 20.3535 L 11.2888 14.8708 C 11.783 14.5687 12.0668 14.2392 12.1675 13.8548 L 12.1675 19.127 C 12.1675 20.1522 12.7716 20.6922 13.4856 20.6922 C 13.8608 20.6922 14.172 20.5732 14.5382 20.3535 L 23.8653 14.8708 C 24.3595 14.5687 24.6433 14.2392 24.744 13.8548 L 24.7439 19.7403 C 24.7439 20.5458 25.1649 20.9668 25.9704 20.9668 L 28.0482 20.9668 C 28.8537 20.9668 29.2656 20.5458 29.2656 19.7403 L 29.2656 7.27371 C 29.2656 6.44077 28.8537 6.05634 28.0482 6.05634 L 25.9704 6.05634 C 25.1649 6.05634 24.7439 6.47739 24.7439 7.27371 L 24.7439 13.15 C 24.6432 12.7747 24.3686 12.4361 23.8652 12.1431 L 14.5381 6.65129 C 14.1628 6.43161 13.8608 6.31262 13.4855 6.31262 C 12.7715 6.31262 12.1674 6.86181 12.1674 7.88697 L 12.1675 13.15 C 12.0668 12.7747 11.783 12.436 11.2887 12.1431 L 1.96169 6.65129 C 1.58649 6.43162 1.28439 6.31263 0.909093 6.31263 C 0.185992 6.31263 -0.40891 6.86182 -0.40891 7.88698 L -0.40891 19.127 C -0.40891 20.1522 0.185992 20.6922 0.909093 20.6922 Z",
    svg_size: Size::new(28.0, 28.0),
    op: PaintOp::Fill,
};
// ...
pub static PLAY_SEQUENTIAL: SvgIcon = SvgIcon {
    svg_path: "M3 8C3 8.55228 2.55228 9 2 9C1.44772 9 1 8.55228 1 8C1 7.44772 1.44772 7 2 7C2.55228 7 3 7.44772 3 8Z M7 8C7 8.55228 6.55228 9 6 9C5.44772 9 5 8.55228 5 8C5 7.44772 5.44772 7 6 7C6.55228 7 7 7.44772 7 8Z M11 8C11 8.55228 10.5523 9 10 9C9.44772 9 9 8.55228 9 8C9 7.44772 9.44772 7 10 7C10.5523 7 11 7.44772 11 8Z M15 8C15 8.55228 14.5523 9 14 9C13.4477 9 13 8.55228 13 8C13 7.44772 13.4477 7 14 7C14.5523 7 15 7.44772 15 8Z",
    svg_size: Size::new(16.0, 16.0),
    op: PaintOp::Fill,
};
// SF Pro Regular - shuffle
pub static PLAY_SHUFFLE: SvgIcon = SvgIcon {
    svg_path: "M1.90792 10.4894C1.90792 10.846 2.17411 11.0971 2.5558 11.0971H3.74107C4.63504 11.0971 5.17746 10.8309 5.78516 10.1077L6.92522 8.75167L8.0452 10.0876C8.66797 10.8309 9.28069 11.1021 10.1797 11.1021H11.0737V12.202C11.0737 12.5033 11.2645 12.6942 11.5709 12.6942C11.7065 12.6942 11.832 12.644 11.9325 12.5636L13.9113 10.9163C14.1574 10.7154 14.1523 10.389 13.9113 10.1881L11.9325 8.53572C11.832 8.45034 11.7065 8.40011 11.5709 8.40011C11.2645 8.40011 11.0737 8.59096 11.0737 8.8923V9.87668H10.2048C9.63728 9.87668 9.28571 9.69085 8.87388 9.20368L7.70871 7.82255L8.87891 6.43136C9.29576 5.93415 9.61719 5.76339 10.1747 5.76339H11.0737V6.76284C11.0737 7.06417 11.2645 7.25502 11.5709 7.25502C11.7065 7.25502 11.832 7.2048 11.9325 7.12444L13.9113 5.47712C14.1574 5.27623 14.1523 4.94978 13.9113 4.74888L11.9325 3.09654C11.832 3.01116 11.7065 2.96094 11.5709 2.96094C11.2645 2.96094 11.0737 3.15179 11.0737 3.45313V4.53795H10.1847C9.25558 4.53795 8.66797 4.79911 8.01004 5.59263L6.92522 6.88337L5.78516 5.53237C5.17746 4.80915 4.59989 4.53795 3.70592 4.53795H2.5558C2.17411 4.53795 1.90792 4.79409 1.90792 5.15067C1.90792 5.50725 2.17913 5.76339 2.5558 5.76339H3.66071C4.20313 5.76339 4.56473 5.9442 4.97656 6.43638L6.13672 7.81752L4.97656 9.20368C4.55971 9.69587 4.23326 9.87668 3.69587 9.87668H2.5558C2.17913 9.87668 1.90792 10.1328 1.90792 10.4894Z",
    svg_size: Size::new(16.0, 20.0),
    op: PaintOp::Fill,
};
// SF Pro Regular - repeat.1
pub static PLAY_LOOP_TRACK: SvgIcon = SvgIcon {
    svg_path: "M13.0273 6.49665C13.3789 6.49665 13.5848 6.30581 13.5848 5.92411V3.56864C13.5848 3.16686 13.3186 2.90067 12.9219 2.90067C12.6004 2.90067 12.4046 3.00112 12.1484 3.19197L11.5006 3.68416C11.3499 3.79967 11.2946 3.91016 11.2946 4.04576C11.2946 4.24665 11.4403 4.40235 11.6663 4.40235C11.7667 4.40235 11.8521 4.36719 11.9375 4.3019L12.4196 3.91016H12.4598V5.92411C12.4598 6.30581 12.6708 6.49665 13.0273 6.49665ZM2.35491 7.37054C2.35491 7.72712 2.62612 7.99833 2.9827 7.99833C3.34431 7.99833 3.61049 7.72712 3.61049 7.37054V7.04409C3.61049 6.22545 4.17299 5.69811 5.02679 5.69811H7.56808V6.74777C7.56808 7.04911 7.75893 7.23996 8.06027 7.23996C8.19587 7.23996 8.32645 7.18974 8.4269 7.10938L10.4057 5.46206C10.6468 5.26116 10.6468 4.93471 10.4057 4.73382L8.4269 3.08148C8.32645 2.9961 8.19587 2.94587 8.06027 2.94587C7.75893 2.94587 7.56808 3.13672 7.56808 3.43806V4.46261H5.12723C3.42467 4.46261 2.35491 5.41183 2.35491 6.92355V7.37054ZM7.19141 8.74665C7.19141 8.44532 7.00056 8.24944 6.69922 8.24944C6.56362 8.24944 6.43304 8.30469 6.33259 8.38505L4.35882 10.0324C4.11272 10.2282 4.11272 10.5547 4.35882 10.7606L6.33259 12.4129C6.43304 12.4983 6.56362 12.5486 6.69922 12.5486C7.00056 12.5486 7.19141 12.3577 7.19141 12.0564V11.0268H10.8677C12.5703 11.0268 13.635 10.0725 13.635 8.56585V8.11886C13.635 7.75726 13.3638 7.48605 13.0073 7.48605C12.6507 7.48605 12.3795 7.75726 12.3795 8.11886V8.44532C12.3795 9.25893 11.822 9.7913 10.9632 9.7913H7.19141V8.74665Z",
    svg_size: Size::new(16.0, 16.0),
    op: PaintOp::Fill,
};
// SF Pro Regular - repeat
pub static PLAY_LOOP_ALL: SvgIcon = SvgIcon {
    svg_path: "M2.35993 7.37053C2.35993 7.71707 2.64118 7.99832 2.98772 7.99832C3.33929 7.99832 3.61551 7.71707 3.61551 7.37053V7.04408C3.61551 6.22545 4.17801 5.6981 5.03181 5.6981H8.80357V6.74777C8.80357 7.04911 8.99442 7.23995 9.30078 7.23995C9.43638 7.23995 9.56194 7.18973 9.66239 7.10937L11.6412 5.46205C11.8873 5.26618 11.8823 4.93471 11.6412 4.73382L9.66239 3.08147C9.56194 2.99609 9.43638 2.94587 9.30078 2.94587C8.99442 2.94587 8.80357 3.13672 8.80357 3.43806V4.46261H5.13225C3.42969 4.46261 2.35993 5.41183 2.35993 6.92355V7.37053ZM7.19643 8.74665C7.19643 8.44531 7.00558 8.24944 6.70424 8.24944C6.56864 8.24944 6.43806 8.30469 6.33761 8.38504L4.36384 10.0324C4.11775 10.2282 4.11775 10.5547 4.36384 10.7606L6.33761 12.4129C6.43806 12.4983 6.56864 12.5485 6.70424 12.5485C7.00558 12.5485 7.19643 12.3577 7.19643 12.0564V11.0268H10.8728C12.5753 11.0268 13.6401 10.0725 13.6401 8.56585V8.11886C13.6401 7.7673 13.3638 7.48605 13.0123 7.48605C12.6657 7.48605 12.3845 7.7673 12.3845 8.11886V8.44531C12.3845 9.25893 11.827 9.79129 10.9682 9.79129H7.19643V8.74665Z",
    svg_size: Size::new(16.0, 16.0),
    op: PaintOp::Fill,
};

// SF Pro Regular - exclamationmark.circle
pub static ERROR: SvgIcon = SvgIcon {
    svg_path: "M13.9912 22.7422C18.9746 22.7422 23.0879 18.6289 23.0879 13.6543C23.0879 8.67969 18.9658 4.56641 13.9824 4.56641C9.00781 4.56641 4.90332 8.67969 4.90332 13.6543C4.90332 18.6289 9.0166 22.7422 13.9912 22.7422ZM13.9912 20.9316C9.95703 20.9316 6.73145 17.6885 6.73145 13.6543C6.73145 9.62012 9.95703 6.38574 13.9824 6.38574C18.0166 6.38574 21.2598 9.62012 21.2686 13.6543C21.2773 17.6885 18.0254 20.9316 13.9912 20.9316ZM13.9824 15.1133C14.4658 15.1133 14.7471 14.8408 14.7559 14.3311L14.8877 10.1035C14.9053 9.58496 14.5186 9.20703 13.9736 9.20703C13.4287 9.20703 13.0508 9.57617 13.0684 10.0947L13.1914 14.3311C13.209 14.832 13.4902 15.1133 13.9824 15.1133ZM13.9824 18.0312C14.5537 18.0312 15.0195 17.6182 15.0195 17.0557C15.0195 16.502 14.5625 16.0889 13.9824 16.0889C13.4111 16.0889 12.9453 16.502 12.9453 17.0557C12.9453 17.6094 13.4199 18.0312 13.9824 18.0312Z",
    svg_size: Size::new(28.0, 28.0),
    op: PaintOp::Fill,
};

// SF Pro Regular - magnifyingglass.circle
pub static SEARCH: SvgIcon = SvgIcon {
    svg_path: "M10.9912 19.7422C15.9746 19.7422 20.0879 15.6289 20.0879 10.6543C20.0879 5.67969 15.9658 1.56641 10.9824 1.56641C6.00781 1.56641 1.90332 5.67969 1.90332 10.6543C1.90332 15.6289 6.0166 19.7422 10.9912 19.7422ZM10.9912 17.9316C6.95703 17.9316 3.73145 14.6885 3.73145 10.6543C3.73145 6.62012 6.95703 3.38574 10.9824 3.38574C15.0166 3.38574 18.2598 6.62012 18.2686 10.6543C18.2773 14.6885 15.0254 17.9316 10.9912 17.9316ZM10.0771 13.3525C10.7539 13.3525 11.3955 13.168 11.9404 12.834L14.0498 14.9346C14.2344 15.1367 14.4541 15.2246 14.7178 15.2246C15.1924 15.2246 15.5352 14.873 15.5352 14.3809C15.5352 14.1611 15.4297 13.9502 15.2715 13.7832L13.1533 11.665C13.5225 11.0938 13.7334 10.417 13.7334 9.69629C13.7334 7.68359 12.0898 6.04004 10.0771 6.04004C8.07324 6.04004 6.4209 7.69238 6.4209 9.69629C6.4209 11.709 8.07324 13.3525 10.0771 13.3525ZM10.0771 12.043C8.79395 12.043 7.72168 10.9883 7.72168 9.69629C7.72168 8.41309 8.79395 7.34961 10.0771 7.34961C11.3691 7.34961 12.4238 8.4043 12.4238 9.69629C12.4238 10.9883 11.3691 12.043 10.0771 12.043Z",
    svg_size: Size::new(22.0, 22.0),
    op: PaintOp::Fill,
};
// SF Pro Regular - heart.circle
pub static HEART: SvgIcon = SvgIcon {
    svg_path: "M11.0879 20.1758C16.0713 20.1758 20.1846 16.0625 20.1846 11.0879C20.1846 6.11328 16.0625 2 11.0791 2C6.10449 2 2 6.11328 2 11.0879C2 16.0625 6.11328 20.1758 11.0879 20.1758ZM11.0879 18.3652C7.05371 18.3652 3.82812 15.1221 3.82812 11.0879C3.82812 7.05371 7.05371 3.81934 11.0791 3.81934C15.1133 3.81934 18.3564 7.05371 18.3652 11.0879C18.374 15.1221 15.1221 18.3652 11.0879 18.3652ZM9.01367 7.29102C7.57227 7.29102 6.5 8.38965 6.5 9.94531C6.5 12.3447 9.10156 14.4277 10.6309 15.3857C10.7803 15.4736 10.9824 15.5967 11.1055 15.5967C11.2285 15.5967 11.4131 15.4736 11.5537 15.3857C13.0654 14.4102 15.6846 12.3447 15.6846 9.94531C15.6846 8.38965 14.6123 7.29102 13.1621 7.29102C12.2305 7.29102 11.501 7.82715 11.0879 8.57422C10.6748 7.82715 9.96289 7.29102 9.01367 7.29102Z",
    svg_size: Size::new(22.0, 22.0),
    op: PaintOp::Fill,
};
// SF Pro Regular - person.crop.circle
pub static ARTIST: SvgIcon = SvgIcon {
    svg_path: "M10.9912 19.7422C15.9746 19.7422 20.0879 15.6289 20.0879 10.6543C20.0879 5.67969 15.9658 1.56641 10.9824 1.56641C6.00781 1.56641 1.90332 5.67969 1.90332 10.6543C1.90332 15.6289 6.0166 19.7422 10.9912 19.7422ZM10.9912 13.6953C8.5127 13.6953 6.58789 14.583 5.65625 15.6025C4.46094 14.3105 3.73145 12.5703 3.73145 10.6543C3.73145 6.62012 6.95703 3.38574 10.9824 3.38574C15.0166 3.38574 18.2598 6.62012 18.2686 10.6543C18.2686 12.5703 17.5391 14.3105 16.335 15.6113C15.4033 14.583 13.4785 13.6953 10.9912 13.6953ZM10.9912 12.2539C12.6963 12.2715 14.0234 10.8125 14.0234 8.93164C14.0234 7.15625 12.6875 5.6709 10.9912 5.6709C9.30371 5.6709 7.95898 7.15625 7.96777 8.93164C7.97656 10.8125 9.29492 12.2451 10.9912 12.2539Z",
    svg_size: Size::new(22.0, 22.0),
    op: PaintOp::Fill,
};
// SF Pro Regular - record.circle
pub static ALBUM: SvgIcon = SvgIcon {
    svg_path: "M10.9912 19.7422C15.9746 19.7422 20.0879 15.6289 20.0879 10.6543C20.0879 5.67969 15.9658 1.56641 10.9824 1.56641C6.00781 1.56641 1.90332 5.67969 1.90332 10.6543C1.90332 15.6289 6.0166 19.7422 10.9912 19.7422ZM10.9912 17.9316C6.95703 17.9316 3.73145 14.6885 3.73145 10.6543C3.73145 6.62012 6.95703 3.38574 10.9824 3.38574C15.0166 3.38574 18.2598 6.62012 18.2686 10.6543C18.2773 14.6885 15.0254 17.9316 10.9912 17.9316ZM11 14.0996C12.9072 14.0996 14.4453 12.5615 14.4453 10.6455C14.4453 8.74707 12.9072 7.2002 11 7.2002C9.08398 7.2002 7.5459 8.74707 7.5459 10.6455C7.5459 12.5615 9.08398 14.0996 11 14.0996Z",
    svg_size: Size::new(22.0, 22.0),
    op: PaintOp::Fill,
};
// SF Pro Regular - list.bullet.circle
pub static PLAYLIST: SvgIcon = SvgIcon {
    svg_path: "M11 19.7334C15.9658 19.7334 20.0791 15.6289 20.0791 10.6543C20.0791 5.68848 15.9658 1.5752 10.9912 1.5752C6.02539 1.5752 1.9209 5.68848 1.9209 10.6543C1.9209 15.6289 6.03418 19.7334 11 19.7334ZM11 17.9492C6.95703 17.9492 3.71387 14.6973 3.71387 10.6543C3.71387 6.61133 6.94824 3.36816 10.9912 3.36816C15.0342 3.36816 18.2861 6.61133 18.2949 10.6543C18.2949 14.6973 15.043 17.9492 11 17.9492ZM6.89551 8.60645C7.31738 8.60645 7.66895 8.25488 7.66895 7.83301C7.66895 7.40234 7.32617 7.05957 6.89551 7.05957C6.46484 7.05957 6.12207 7.40234 6.12207 7.83301C6.12207 8.26367 6.46484 8.60645 6.89551 8.60645ZM9.01367 8.39551H15.2891C15.6055 8.39551 15.8516 8.14941 15.8516 7.83301C15.8516 7.5166 15.6055 7.27051 15.2891 7.27051H9.01367C8.69727 7.27051 8.45117 7.5166 8.45117 7.83301C8.45117 8.14062 8.70605 8.39551 9.01367 8.39551ZM6.89551 11.4365C7.31738 11.4365 7.66895 11.0762 7.66895 10.6543C7.66895 10.2324 7.31738 9.88086 6.89551 9.88086C6.46484 9.88086 6.12207 10.2324 6.12207 10.6543C6.12207 11.085 6.46484 11.4365 6.89551 11.4365ZM9.01367 11.2168H15.2891C15.6055 11.2168 15.8516 10.9795 15.8516 10.6543C15.8516 10.3379 15.6055 10.1006 15.2891 10.1006H9.01367C8.70605 10.1006 8.45117 10.3467 8.45117 10.6543C8.45117 10.9707 8.70605 11.2168 9.01367 11.2168ZM6.89551 14.2578C7.32617 14.2578 7.66895 13.915 7.66895 13.4844C7.66895 13.0625 7.31738 12.7109 6.89551 12.7109C6.46484 12.7109 6.12207 13.0537 6.12207 13.4844C6.12207 13.915 6.46484 14.2578 6.89551 14.2578ZM9.01367 14.0469H15.2891C15.6055 14.0469 15.8516 13.8008 15.8516 13.4844C15.8516 13.168 15.6055 12.9219 15.2891 12.9219H9.01367C8.70605 12.9219 8.45117 13.1768 8.45117 13.4844C8.45117 13.8008 8.69727 14.0469 9.01367 14.0469Z",
    svg_size: Size::new(22.0, 22.0),
    op: PaintOp::Fill,
};
// SFSymbols - house
pub static HOME: SvgIcon = SvgIcon {
    svg_path: "M17.2 41.5H8.9c-2 0-2.8-.8-2.8-2.8V16l16-14.6c.3-.3.6-.5 1-.5s.7.2 1 .5L40 16V38.8c0 2-.8 2.8-2.8 2.8H28.9V26.2c0-.7-.5-1.2-1.2-1.2H18.4c-.7 0-1.2.5-1.2 1.2Zm20.1 1.2c2.5 0 3.7-1.3 3.7-3.7V17l4.1 3.7c.2.1.3.2.5.2.3 0 .5-.2.5-.4s0-.3-.2-.4L24.9.7c-.5-.5-1.1-.8-1.8-.8s-1.3.3-1.8.8L8.6 12.4V.6c0-.5-.4-.8-.8-.8H6.9c-.5 0-.8.3-.8.8v14l-6 5.5c-.1.1-.2.2-.2.4s.2.4.5.4.4-.1.5-.2L5 16.9v22c0 2.4 1.3 3.7 3.7 3.7H37.4Z",
    svg_size: Size::new(46.0, 46.0),
    op: PaintOp::Fill,
};
// SF Pro Regular - mic.circle
pub static PODCAST: SvgIcon = SvgIcon {
    svg_path: "M10.9957 20C15.9285 20 20 15.9265 20 11C20 6.0735 15.9198 2 10.987 2C6.06283 2 2 6.0735 2 11C2 15.9265 6.07153 20 10.9957 20ZM10.9957 18.207C7.00242 18.207 3.80957 14.9952 3.80957 11C3.80957 7.00484 7.00242 3.80174 10.987 3.80174C14.9802 3.80174 18.1904 7.00484 18.1991 11C18.2078 14.9952 14.9889 18.207 10.9957 18.207ZM10.9957 12.5928C11.8395 12.5928 12.4746 11.9313 12.4746 11.0348V7.42263C12.4746 6.51741 11.8395 5.8646 10.9957 5.8646C10.1431 5.8646 9.50797 6.51741 9.50797 7.42263V11.0348C9.50797 11.9313 10.1431 12.5928 10.9957 12.5928ZM8.82939 16.1789H13.1619C13.4316 16.1789 13.6752 15.9439 13.6752 15.6741C13.6752 15.3956 13.4403 15.1605 13.1619 15.1605H11.5002V14.3598C13.2228 14.1509 14.4234 12.854 14.4234 11.087V10.0077C14.4234 9.73791 14.1885 9.51161 13.9188 9.51161C13.6404 9.51161 13.4055 9.73791 13.4055 10.0077V11.0783C13.4055 12.4536 12.3876 13.4371 10.987 13.4371C9.59497 13.4371 8.57709 12.4536 8.57709 11.0783V10.0077C8.57709 9.73791 8.34219 9.51161 8.0638 9.51161C7.7941 9.51161 7.55921 9.73791 7.55921 10.0077V11.087C7.55921 12.854 8.76849 14.1596 10.4911 14.3598V15.1605H8.82939C8.55099 15.1605 8.30739 15.3956 8.30739 15.6741C8.30739 15.9526 8.55099 16.1789 8.82939 16.1789Z",
    svg_size: Size::new(22.0, 22.0),
    op: PaintOp::Fill,
};
// Bootstrap - explicit-fill
pub static EXPLICIT: SvgIcon = SvgIcon {
    svg_path: "M3.75 0A3.75 3.75 0 0 0 0 3.75v16.5A3.75 3.75 0 0 0 3.75 24h16.5A3.75 3.75 0 0 0 24 20.25V3.75A3.75 3.75 0 0 0 20.25 0Zm6.49 16.32h5.51V18h-7.5V6h7.5v1.68h-5.51v3.42h5.18v1.6h-5.18v3.62z",
    svg_size: Size::new(24.0, 24.0),
    op: PaintOp::Fill,
};
// SF Pro Regular - plus.circle
pub static CIRCLE_PLUS: SvgIcon = SvgIcon {
    svg_path: "M11.9531 23.9062C18.4922 23.9062 23.9062 18.4805 23.9062 11.9531C23.9062 5.41406 18.4805 0 11.9414 0C5.41406 0 0 5.41406 0 11.9531C0 18.4805 5.42578 23.9062 11.9531 23.9062ZM11.9531 21.9141C6.42188 21.9141 2.00391 17.4844 2.00391 11.9531C2.00391 6.42188 6.41016 1.99219 11.9414 1.99219C17.4727 1.99219 21.9141 6.42188 21.9141 11.9531C21.9141 17.4844 17.4844 21.9141 11.9531 21.9141ZM6.51562 11.9531C6.51562 12.5273 6.91406 12.9141 7.51172 12.9141L10.957 12.9141L10.957 16.3711C10.957 16.957 11.3555 17.3672 11.9297 17.3672C12.5156 17.3672 12.9258 16.9688 12.9258 16.3711L12.9258 12.9141L16.3828 12.9141C16.9688 12.9141 17.3789 12.5273 17.3789 11.9531C17.3789 11.3672 16.9688 10.957 16.3828 10.957L12.9258 10.957L12.9258 7.51172C12.9258 6.91406 12.5156 6.50391 11.9297 6.50391C11.3555 6.50391 10.957 6.91406 10.957 7.51172L10.957 10.957L7.51172 10.957C6.91406 10.957 6.51562 11.3672 6.51562 11.9531Z",
    svg_size: Size::new(24.0, 24.0),
    op: PaintOp::Fill,
};

// SF Pro Regular - checkmark.circle.fill
pub static CIRCLE_CHECK: SvgIcon = SvgIcon {
    svg_path: "M23.9062 11.9531C23.9062 18.4805 18.4922 23.9062 11.9531 23.9062C5.42578 23.9062 0 18.4805 0 11.9531C0 5.41406 5.41406 0 11.9414 0C18.4805 0 23.9062 5.41406 23.9062 11.9531ZM15.5977 7.30078L10.5938 15.3398L8.21484 12.2695C7.92188 11.8828 7.66406 11.7773 7.32422 11.7773C6.79688 11.7773 6.38672 12.2109 6.38672 12.7383C6.38672 13.0078 6.49219 13.2656 6.66797 13.5L9.60938 17.1094C9.91406 17.5195 10.2422 17.6836 10.6406 17.6836C11.0391 17.6836 11.3789 17.4961 11.625 17.1094L17.1328 8.4375C17.2734 8.19141 17.4258 7.92188 17.4258 7.66406C17.4258 7.11328 16.9453 6.76172 16.4297 6.76172C16.125 6.76172 15.8203 6.94922 15.5977 7.30078Z",
    svg_size: Size::new(24.0, 24.0),
    op: PaintOp::Fill,
};

#[derive(Copy, Clone)]
pub enum PaintOp {
    Fill,
}

#[derive(Clone)]
pub struct SvgIcon {
    svg_path: &'static str,
    svg_size: Size,
    op: PaintOp,
}

impl SvgIcon {
    pub fn scale(&self, to_size: impl Into<Size>) -> Icon {
        let to_size = to_size.into();
        let bez_path = BezPath::from_svg(self.svg_path).expect("Failed to parse SVG");
        let scale = Affine::scale_non_uniform(
            to_size.width / self.svg_size.width,
            to_size.height / self.svg_size.height,
        );
        Icon::new(self.op, bez_path, to_size, scale)
    }
}

#[derive(Clone)]
pub struct Icon {
    op: PaintOp,
    bez_path: BezPath,
    size: Size,
    scale: Affine,
    color: KeyOrValue<Color>,
}

impl Icon {
    pub fn new(op: PaintOp, bez_path: BezPath, size: Size, scale: Affine) -> Self {
        Icon {
            op,
            bez_path,
            size,
            scale,
            color: theme::ICON_COLOR.into(),
        }
    }

    pub fn with_color(mut self, color: impl Into<KeyOrValue<Color>>) -> Self {
        self.set_color(color);
        self
    }

    pub fn set_color(&mut self, color: impl Into<KeyOrValue<Color>>) {
        self.color = color.into();
    }
}

impl<T> Widget<T> for Icon {
    fn event(&mut self, _ctx: &mut EventCtx, _ev: &Event, _data: &mut T, _env: &Env) {}

    fn lifecycle(&mut self, _ctx: &mut LifeCycleCtx, _ev: &LifeCycle, _data: &T, _env: &Env) {}

    fn update(&mut self, _ctx: &mut UpdateCtx, _old_data: &T, _data: &T, _env: &Env) {}

    fn layout(&mut self, _ctx: &mut LayoutCtx, bc: &BoxConstraints, _data: &T, _env: &Env) -> Size {
        bc.constrain(self.size)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, _data: &T, env: &Env) {
        let color = self.color.resolve(env);
        ctx.with_save(|ctx| {
            ctx.transform(self.scale);
            match self.op {
                PaintOp::Fill => ctx.fill(&self.bez_path, &color),
            }
        });
    }
}
