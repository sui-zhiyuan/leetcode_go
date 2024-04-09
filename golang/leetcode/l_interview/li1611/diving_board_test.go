package li1611

import (
	"testing"

	"github.com/google/go-cmp/cmp"
)

func Test_divingBoard(t *testing.T) {
	type args struct {
		shorter int
		longer  int
		k       int
	}
	tests := []struct {
		name string
		args args
		want []int
	}{
		{
			name: "case 1",
			args: args{
				shorter: 1,
				longer:  2,
				k:       3,
			},
			want: []int{3, 4, 5, 6},
		},
		{
			name: "case 2",
			args: args{
				shorter: 1,
				longer:  2,
				k:       0,
			},
			want: []int{}, // why 0 doesn't count as a length ?
		},
		{
			name: "case 3",
			args: args{
				shorter: 2,
				longer:  4,
				k:       3,
			},
			want: []int{6, 8, 10, 12},
		},
		{
			name: "case 4",
			args: args{
				shorter: 2,
				longer:  1118596,
				k:       979,
			},
			want: []int{
				1958, 1120552, 2239146, 3357740, 4476334, 5594928, 6713522, 7832116, 8950710, 10069304,
				11187898, 12306492, 13425086, 14543680, 15662274, 16780868, 17899462, 19018056, 20136650,
				21255244, 22373838, 23492432, 24611026, 25729620, 26848214, 27966808, 29085402, 30203996,
				31322590, 32441184, 33559778, 34678372, 35796966, 36915560, 38034154, 39152748, 40271342,
				41389936, 42508530, 43627124, 44745718, 45864312, 46982906, 48101500, 49220094, 50338688,
				51457282, 52575876, 53694470, 54813064, 55931658, 57050252, 58168846, 59287440, 60406034,
				61524628, 62643222, 63761816, 64880410, 65999004, 67117598, 68236192, 69354786, 70473380,
				71591974, 72710568, 73829162, 74947756, 76066350, 77184944, 78303538, 79422132, 80540726,
				81659320, 82777914, 83896508, 85015102, 86133696, 87252290, 88370884, 89489478, 90608072,
				91726666, 92845260, 93963854, 95082448, 96201042, 97319636, 98438230, 99556824, 100675418,
				101794012, 102912606, 104031200, 105149794, 106268388, 107386982, 108505576, 109624170, 110742764,
				111861358, 112979952, 114098546, 115217140, 116335734, 117454328, 118572922, 119691516, 120810110,
				121928704, 123047298, 124165892, 125284486, 126403080, 127521674, 128640268, 129758862, 130877456,
				131996050, 133114644, 134233238, 135351832, 136470426, 137589020, 138707614, 139826208, 140944802,
				142063396, 143181990, 144300584, 145419178, 146537772, 147656366, 148774960, 149893554, 151012148,
				152130742, 153249336, 154367930, 155486524, 156605118, 157723712, 158842306, 159960900, 161079494,
				162198088, 163316682, 164435276, 165553870, 166672464, 167791058, 168909652, 170028246, 171146840,
				172265434, 173384028, 174502622, 175621216, 176739810, 177858404, 178976998, 180095592, 181214186,
				182332780, 183451374, 184569968, 185688562, 186807156, 187925750, 189044344, 190162938, 191281532,
				192400126, 193518720, 194637314, 195755908, 196874502, 197993096, 199111690, 200230284, 201348878,
				202467472, 203586066, 204704660, 205823254, 206941848, 208060442, 209179036, 210297630, 211416224,
				212534818, 213653412, 214772006, 215890600, 217009194, 218127788, 219246382, 220364976, 221483570,
				222602164, 223720758, 224839352, 225957946, 227076540, 228195134, 229313728, 230432322, 231550916,
				232669510, 233788104, 234906698, 236025292, 237143886, 238262480, 239381074, 240499668, 241618262,
				242736856, 243855450, 244974044, 246092638, 247211232, 248329826, 249448420, 250567014, 251685608,
				252804202, 253922796, 255041390, 256159984, 257278578, 258397172, 259515766, 260634360, 261752954,
				262871548, 263990142, 265108736, 266227330, 267345924, 268464518, 269583112, 270701706, 271820300,
				272938894, 274057488, 275176082, 276294676, 277413270, 278531864, 279650458, 280769052, 281887646,
				283006240, 284124834, 285243428, 286362022, 287480616, 288599210, 289717804, 290836398, 291954992,
				293073586, 294192180, 295310774, 296429368, 297547962, 298666556, 299785150, 300903744, 302022338,
				303140932, 304259526, 305378120, 306496714, 307615308, 308733902, 309852496, 310971090, 312089684,
				313208278, 314326872, 315445466, 316564060, 317682654, 318801248, 319919842, 321038436, 322157030,
				323275624, 324394218, 325512812, 326631406, 327750000, 328868594, 329987188, 331105782, 332224376,
				333342970, 334461564, 335580158, 336698752, 337817346, 338935940, 340054534, 341173128, 342291722,
				343410316, 344528910, 345647504, 346766098, 347884692, 349003286, 350121880, 351240474, 352359068,
				353477662, 354596256, 355714850, 356833444, 357952038, 359070632, 360189226, 361307820, 362426414,
				363545008, 364663602, 365782196, 366900790, 368019384, 369137978, 370256572, 371375166, 372493760,
				373612354, 374730948, 375849542, 376968136, 378086730, 379205324, 380323918, 381442512, 382561106,
				383679700, 384798294, 385916888, 387035482, 388154076, 389272670, 390391264, 391509858, 392628452,
				393747046, 394865640, 395984234, 397102828, 398221422, 399340016, 400458610, 401577204, 402695798,
				403814392, 404932986, 406051580, 407170174, 408288768, 409407362, 410525956, 411644550, 412763144,
				413881738, 415000332, 416118926, 417237520, 418356114, 419474708, 420593302, 421711896, 422830490,
				423949084, 425067678, 426186272, 427304866, 428423460, 429542054, 430660648, 431779242, 432897836,
				434016430, 435135024, 436253618, 437372212, 438490806, 439609400, 440727994, 441846588, 442965182,
				444083776, 445202370, 446320964, 447439558, 448558152, 449676746, 450795340, 451913934, 453032528,
				454151122, 455269716, 456388310, 457506904, 458625498, 459744092, 460862686, 461981280, 463099874,
				464218468, 465337062, 466455656, 467574250, 468692844, 469811438, 470930032, 472048626, 473167220,
				474285814, 475404408, 476523002, 477641596, 478760190, 479878784, 480997378, 482115972, 483234566,
				484353160, 485471754, 486590348, 487708942, 488827536, 489946130, 491064724, 492183318, 493301912,
				494420506, 495539100, 496657694, 497776288, 498894882, 500013476, 501132070, 502250664, 503369258,
				504487852, 505606446, 506725040, 507843634, 508962228, 510080822, 511199416, 512318010, 513436604,
				514555198, 515673792, 516792386, 517910980, 519029574, 520148168, 521266762, 522385356, 523503950,
				524622544, 525741138, 526859732, 527978326, 529096920, 530215514, 531334108, 532452702, 533571296,
				534689890, 535808484, 536927078, 538045672, 539164266, 540282860, 541401454, 542520048, 543638642,
				544757236, 545875830, 546994424, 548113018, 549231612, 550350206, 551468800, 552587394, 553705988,
				554824582, 555943176, 557061770, 558180364, 559298958, 560417552, 561536146, 562654740, 563773334,
				564891928, 566010522, 567129116, 568247710, 569366304, 570484898, 571603492, 572722086, 573840680,
				574959274, 576077868, 577196462, 578315056, 579433650, 580552244, 581670838, 582789432, 583908026,
				585026620, 586145214, 587263808, 588382402, 589500996, 590619590, 591738184, 592856778, 593975372,
				595093966, 596212560, 597331154, 598449748, 599568342, 600686936, 601805530, 602924124, 604042718,
				605161312, 606279906, 607398500, 608517094, 609635688, 610754282, 611872876, 612991470, 614110064,
				615228658, 616347252, 617465846, 618584440, 619703034, 620821628, 621940222, 623058816, 624177410,
				625296004, 626414598, 627533192, 628651786, 629770380, 630888974, 632007568, 633126162, 634244756,
				635363350, 636481944, 637600538, 638719132, 639837726, 640956320, 642074914, 643193508, 644312102,
				645430696, 646549290, 647667884, 648786478, 649905072, 651023666, 652142260, 653260854, 654379448,
				655498042, 656616636, 657735230, 658853824, 659972418, 661091012, 662209606, 663328200, 664446794,
				665565388, 666683982, 667802576, 668921170, 670039764, 671158358, 672276952, 673395546, 674514140,
				675632734, 676751328, 677869922, 678988516, 680107110, 681225704, 682344298, 683462892, 684581486,
				685700080, 686818674, 687937268, 689055862, 690174456, 691293050, 692411644, 693530238, 694648832,
				695767426, 696886020, 698004614, 699123208, 700241802, 701360396, 702478990, 703597584, 704716178,
				705834772, 706953366, 708071960, 709190554, 710309148, 711427742, 712546336, 713664930, 714783524,
				715902118, 717020712, 718139306, 719257900, 720376494, 721495088, 722613682, 723732276, 724850870,
				725969464, 727088058, 728206652, 729325246, 730443840, 731562434, 732681028, 733799622, 734918216,
				736036810, 737155404, 738273998, 739392592, 740511186, 741629780, 742748374, 743866968, 744985562,
				746104156, 747222750, 748341344, 749459938, 750578532, 751697126, 752815720, 753934314, 755052908,
				756171502, 757290096, 758408690, 759527284, 760645878, 761764472, 762883066, 764001660, 765120254,
				766238848, 767357442, 768476036, 769594630, 770713224, 771831818, 772950412, 774069006, 775187600,
				776306194, 777424788, 778543382, 779661976, 780780570, 781899164, 783017758, 784136352, 785254946,
				786373540, 787492134, 788610728, 789729322, 790847916, 791966510, 793085104, 794203698, 795322292,
				796440886, 797559480, 798678074, 799796668, 800915262, 802033856, 803152450, 804271044, 805389638,
				806508232, 807626826, 808745420, 809864014, 810982608, 812101202, 813219796, 814338390, 815456984,
				816575578, 817694172, 818812766, 819931360, 821049954, 822168548, 823287142, 824405736, 825524330,
				826642924, 827761518, 828880112, 829998706, 831117300, 832235894, 833354488, 834473082, 835591676,
				836710270, 837828864, 838947458, 840066052, 841184646, 842303240, 843421834, 844540428, 845659022,
				846777616, 847896210, 849014804, 850133398, 851251992, 852370586, 853489180, 854607774, 855726368,
				856844962, 857963556, 859082150, 860200744, 861319338, 862437932, 863556526, 864675120, 865793714,
				866912308, 868030902, 869149496, 870268090, 871386684, 872505278, 873623872, 874742466, 875861060,
				876979654, 878098248, 879216842, 880335436, 881454030, 882572624, 883691218, 884809812, 885928406,
				887047000, 888165594, 889284188, 890402782, 891521376, 892639970, 893758564, 894877158, 895995752,
				897114346, 898232940, 899351534, 900470128, 901588722, 902707316, 903825910, 904944504, 906063098,
				907181692, 908300286, 909418880, 910537474, 911656068, 912774662, 913893256, 915011850, 916130444,
				917249038, 918367632, 919486226, 920604820, 921723414, 922842008, 923960602, 925079196, 926197790,
				927316384, 928434978, 929553572, 930672166, 931790760, 932909354, 934027948, 935146542, 936265136,
				937383730, 938502324, 939620918, 940739512, 941858106, 942976700, 944095294, 945213888, 946332482,
				947451076, 948569670, 949688264, 950806858, 951925452, 953044046, 954162640, 955281234, 956399828,
				957518422, 958637016, 959755610, 960874204, 961992798, 963111392, 964229986, 965348580, 966467174,
				967585768, 968704362, 969822956, 970941550, 972060144, 973178738, 974297332, 975415926, 976534520,
				977653114, 978771708, 979890302, 981008896, 982127490, 983246084, 984364678, 985483272, 986601866,
				987720460, 988839054, 989957648, 991076242, 992194836, 993313430, 994432024, 995550618, 996669212,
				997787806, 998906400, 1000024994, 1001143588, 1002262182, 1003380776, 1004499370, 1005617964, 1006736558,
				1007855152, 1008973746, 1010092340, 1011210934, 1012329528, 1013448122, 1014566716, 1015685310, 1016803904,
				1017922498, 1019041092, 1020159686, 1021278280, 1022396874, 1023515468, 1024634062, 1025752656, 1026871250,
				1027989844, 1029108438, 1030227032, 1031345626, 1032464220, 1033582814, 1034701408, 1035820002, 1036938596,
				1038057190, 1039175784, 1040294378, 1041412972, 1042531566, 1043650160, 1044768754, 1045887348, 1047005942,
				1048124536, 1049243130, 1050361724, 1051480318, 1052598912, 1053717506, 1054836100, 1055954694, 1057073288,
				1058191882, 1059310476, 1060429070, 1061547664, 1062666258, 1063784852, 1064903446, 1066022040, 1067140634,
				1068259228, 1069377822, 1070496416, 1071615010, 1072733604, 1073852198, 1074970792, 1076089386, 1077207980,
				1078326574, 1079445168, 1080563762, 1081682356, 1082800950, 1083919544, 1085038138, 1086156732, 1087275326,
				1088393920, 1089512514, 1090631108, 1091749702, 1092868296, 1093986890, 1095105484},
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := divingBoard(tt.args.shorter, tt.args.longer, tt.args.k); !cmp.Equal(got, tt.want) {
				t.Errorf("divingBoard() = %v, want %v\ndiff=%v", got, tt.want, cmp.Diff(got, tt.want))
			}
		})
	}
}