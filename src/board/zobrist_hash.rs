use crate::game::game_flags::GameFlags;

use super::{player_color::PlayerColor, dynamic_state::DynamicState, piece_board::PieceBoard, piece_type::ColoredPieceType, square::{VALID_SQUARES}};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ZobristHash {
    pub hash: u64,
}

impl ZobristHash {
    pub fn from_position(piece_board: &PieceBoard, flags: GameFlags) -> ZobristHash {
        let mut hash = ZobristHash::empty();

        for s in VALID_SQUARES {
            if piece_board[s] != ColoredPieceType::None {
                hash.add_piece(piece_board[s], s);
            }
        }

        hash.toggle_flags(flags);

        return hash;
    }
}

impl ZobristHash {
    pub fn toggle_flags(&mut self, flags: GameFlags) {
        self.hash ^= if flags.white_king_side_castle { WKC_HASH } else { 0 };
        self.hash ^= if flags.white_queen_side_castle { WQC_HASH } else { 0 };
        self.hash ^= if flags.black_king_side_castle { BKC_HASH } else { 0 };
        self.hash ^= if flags.black_queen_side_castle { BQC_HASH } else { 0 };
        self.hash ^= match flags.active_color {
            PlayerColor::White => TURN_HASH,
            PlayerColor::Black => 0,
        };
        self.hash ^= EN_PASSANT[flags.en_passant_square as usize];
    }
}

impl DynamicState for ZobristHash {
    fn empty() -> Self {
        ZobristHash {
            hash: 0,
        }
    }

    fn add_piece(&mut self, pt: ColoredPieceType, s: i8) {
        self.hash ^= SQUARE_PIECE_HASHS[s as usize][pt as usize];
    }

    fn remove_piece(&mut self, pt: ColoredPieceType, s: i8) {
        self.hash ^= SQUARE_PIECE_HASHS[s as usize][pt as usize];
    }
}

const WQC_HASH: u64 = 12420616827152515493;
const WKC_HASH: u64 = 8371556456389286746;
const BQC_HASH: u64 = 17746852737385052894;
const BKC_HASH: u64 = 8869386327382603398;
const TURN_HASH: u64 = 8521608624400063290;

const EN_PASSANT: [u64; 65] = [
        0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000,
        0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000,
        0xd6405f892fef003e, 0xa1a5091fe8b85b7f, 0x3b7f9acec30e842c, 0x1e1a71ef88e11b18, 0x416f21b972e14c98, 0x19566d456753449f, 0x01b086daa3424a31, 0x42fe0c0eb8fd7b38,
        0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000,
        0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000,
        0x51c1a5ea0dfaaed2, 0xada5f2016cdb0abf, 0xaaa2f9591258fdc0, 0x6ba266d58f0ff2dc, 0x98dac5bb38ec3250, 0x652a878b566f0cee, 0xbb21eb1d25bf8aa0, 0xaa681e82d8e5564b,
        0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000,
        0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000,
        0
];

const SQUARE_PIECE_HASHS: [[u64; 12]; 64] = [
[3039665143350635744, 17092169764834922902, 3925853326203578338, 17354356390057816443, 7472514735885487017, 15392575389892135373, 6651258979722590487, 7954050523632553952, 4091066645749342542, 7367789944430992549, 11178490497920601604, 15053050127913984131],
[4887722226293496868, 5982858118796997078, 8576238699864126023, 9201958687904771367, 17261754958921504760, 12567524518805675227, 1893335993551434743, 15601433409731891584, 16247571489014550244, 14777101723219538124, 10128132881256468928, 13721866742607316403],
[432083089116527842, 1619745982539918128, 6706165792680712556, 8733238636942794458, 1588620480596059980, 554662084526220068, 11996451473773378344, 13744791726981688833, 4267635528192992769, 15275958008117617963, 10848168429927460833, 13834226467308377224],
[14858267927369342187, 17757263843947683420, 16107017628676374209, 18170679730441966118, 9529434117178407452, 1622010035731884788, 2551232692820157863, 596753025190769240, 14024448974176499624, 15647636381494939626, 16021419778252164261, 18441888775405676678],
[1818823088795070159, 17398681883424871134, 17426970971547820824, 2503079508015407498, 8723189764669060319, 12355957506211230239, 13145057724982000008, 9889976522172206870, 14385903368994088063, 2832352739271835549, 6600520397499542232, 13503853511978166114],
[9641299441002630886, 937906100070043690, 13716224274401458573, 9820719156275294914, 16747057234010095325, 15464293583635412378, 9437192333974783771, 3662121894795375953, 12075246326810753142, 10872321040590727668, 9540139986157588363, 694174224147308566],
[9381096007824274448, 14824892985129074558, 16139411311397955041, 3282359954775590729, 10481090228394943594, 1866945705709910233, 11404666316110551278, 1365656485232507858, 16146026727600967190, 3888023405762979757, 6821175244508058020, 486383944193203661],
[13409798258698656936, 13525322135230780835, 2616316835603200701, 11315312042551450376, 15278291941328699266, 2908451273383349474, 2944294118174535763, 9061156129495999344, 4644764316443054696, 8173180343330619392, 11303377631297766531, 2037559442604394789],
[1562227445955881600, 7046531995664840542, 16192214243462374185, 1155946191946376980, 15023982570842420656, 7941933429884250686, 9595375417434268570, 8576846849120160442, 16713397712283611116, 16277304554884355250, 6919279552059237716, 8545779556791568555],
[7279404108581901610, 12235346462863532717, 12069510223841165632, 2073812333444554879, 7171010147281261544, 6380331852116124553, 9868296943091237242, 5814040969984356950, 10262018911853032223, 3972222731460935445, 14510025088006086826, 10524008089147947838],
[11027271255197654710, 15288747017783743730, 4074876187051667684, 18225351105944330095, 6597021759010133054, 832307845367842501, 16599987238882988214, 8394325194395049970, 18012265665714345747, 1099893803435312801, 15256945095413623349, 1727977525774942558],
[15311711152231598787, 14958072438174717919, 10383747747154039964, 11989377372001507212, 11110213264710956622, 10234813090631856758, 12111499154293728723, 8466747823585595390, 2742921995524000201, 12235986095995676025, 3343624773756071820, 16567443036232131842],
[3944702977097533982, 1271301877903338489, 2964301336114302538, 14950256939287190195, 6904011027885783379, 5931503956396002531, 8768712881300032083, 1865327468140426728, 4827253020903941846, 4400340832607052763, 10701407752195245619, 956635692188044283],
[8118061797330058161, 14461878779976148602, 11751074306213153781, 6808094413309291485, 7886941534737077127, 2265711721332821984, 8480668498290010020, 1620139821991426059, 13349224698355829465, 16817230175946124964, 5549671495741438904, 4830526900632295276],
[9142332358781574662, 4097007774221683662, 13065798050659238070, 17124670839689736921, 7743089476317993670, 10235976883200768899, 2925691794561753358, 2949935847169477504, 6264691206665264304, 2281575449638138277, 9461501277554032187, 8580699628588080758],
[16172332670039321944, 5859712353390722520, 2159106020555588562, 10659766174005770544, 4154833785644311671, 11638855189565779714, 1356223428969006104, 5909557745836321086, 12688880051872818991, 754508634162745896, 16990431595559805963, 3052170878705327432],
[5110218906206276443, 16843689220559065616, 7958381970421163058, 188312068391704355, 16528482361703130464, 16635990842196411439, 15025979820913100736, 7549901311679780751, 7931771029733070679, 14215267416590659905, 11181463307130994917, 12011343715219498519],
[2320780388441780959, 13554351855065452145, 13915621814424930047, 14125579104750316295, 7524185644050173247, 9927435960776416489, 17158690142561241914, 893778621271441347, 4314605518452410935, 2000071259635120755, 6723216941082519194, 4655442569359862560],
[14142509248857599976, 6909132067854689482, 11628724052185587828, 11891879614955168627, 13297974970434746960, 3591013829620143912, 11275691280223713262, 16448221475558522618, 1045695966966692279, 12563613498482473827, 7631050393676339093, 7693509131375238090],
[12387221576009901097, 11647599645094519422, 2066177679810019473, 4587784268396669225, 14622084187058472404, 5731461481058643635, 2449458175703664849, 17891052551469153295, 1598331188958640953, 12462254651308231844, 6619154859709048771, 9615694061392346947],
[6204219852621757017, 17799553548421568586, 11402252299713645032, 1725015579499966733, 2119866095333453348, 5506809196365509101, 2839763799767583311, 9352316860363759596, 3201062842986906879, 18204424274327864434, 7234041144873453968, 7263631902373223355],
[11333647666447395763, 17115529526927421592, 4126297850782445956, 8362252898913587583, 8653366199711104172, 6117732154581950202, 14589747878486569063, 2140968907316515913, 5281849625916574093, 7797605358877044163, 7303487011300835697, 7459017158050065308],
[16470363626466082143, 271162793187920166, 15499315009850086940, 17844157592547271523, 1728910833124413185, 8829916113739268524, 16169048726285753048, 9445229682005843401, 1063409532141608431, 10577583615382786706, 10812353418770569033, 1846119927217585962],
[12639213005569962426, 9716046125845874338, 13239357928910245521, 13200748684899688470, 8910762099371825124, 14465190489512241331, 9841860153799500312, 6291397663447193371, 17036036208220045259, 14309362720166905965, 6277891070189930031, 966091815846129775],
[3186757644825886021, 129917528258828500, 16824041466984078974, 11640387479893564958, 70728030974477495, 7488103787138412877, 14828869748843478382, 10950342504263479853, 9836967559522155059, 3746285090723908582, 1184868944690495963, 12714949550479931247],
[17454886500926034901, 2739584168423902216, 4895866487451221352, 6826221877195113004, 18316111080870099243, 8935159565958822137, 12512504998957492476, 17414616131224353314, 8078310785055320827, 13954769831425182219, 3311202025183474818, 9116475713979397785],
[2732301430432697104, 14023974423717679390, 7340257033986365099, 17573320705652098536, 6144173190033021603, 8637328940624067326, 8253600695008936316, 18321370795237964342, 15616595271741154617, 8237865477055504529, 7134923289786314470, 2464289676284694993],
[13193717344002225144, 13404782445710034128, 6818008715618943193, 15779006913875236494, 2024091102894900590, 2297986568710416366, 1089555812251473009, 16401594061439437479, 7096152724565906174, 14360577871696367880, 14542691362813197352, 7405816510293179165],
[6570732136845949321, 7025221408316118321, 16041837002877441074, 78227669413203068, 5077323590857287084, 2688013107207521921, 5857947283519946899, 525898001949264974, 14828718930654762214, 18077522240532112182, 6939940998255971309, 2146497882801672355],
[374781497034450274, 12844944966642889465, 15712229676611252764, 16061375755347004844, 14668136072952902744, 14497153118440954127, 1475288062474323471, 18390722724877670732, 6610497762087865168, 504760337672098303, 16174018810824648457, 12830793114120922560],
[1915522408851958492, 17375536343629511295, 359078500708623627, 17125057783868599628, 8023023643753573938, 147383349392888502, 17520131730918510809, 8793260902697359861, 5919820762240597150, 12100881712597025582, 17540659717724954725, 7002993163925591573],
[5381749042683912869, 700454459965139953, 16766093298231069864, 16122189120231787740, 9369403436621578102, 7811335761763529038, 9825990047981131088, 8679723246306061529, 14728243492498574497, 10545407421134391987, 6498898126518734727, 10843460129102715672],
[9748004019792533378, 16664837471609519558, 4168032247120937572, 16052017977895900837, 1173686017927148725, 4148212377933555576, 7257688051584686683, 17354733182699833123, 1882734133538181949, 1623672783655677935, 11522024061314891492, 15321380840194407606],
[13421736829303712685, 6879152973447253954, 7801074844904686979, 4926363771639438556, 7776434469735563887, 14620645896615164919, 17328132063408565563, 15325088737397682817, 6258731910469955807, 3925022221906168452, 16468590858907513477, 6383698226085155351],
[4447365961110882720, 14990776206274907905, 400808656217305242, 11491170637278562262, 5015111149340954885, 13839684212467849313, 975112359718655207, 8135353012690678427, 11007972398548489322, 14825146017352593630, 6884762059087715642, 1491949845867033790],
[10796273231397775561, 7562230223541420704, 15534163775344790158, 10067567512698011029, 15296155818571805159, 13705144645686335295, 12312095249667911379, 12429695788354195843, 9181700924997171967, 2804719549288736586, 14537735570692579107, 6781913140051468216],
[5486876819135697030, 4012313268142747285, 9903815702162498227, 2274941822595212986, 5732985820400071616, 462686914055243079, 146064484205687835, 8164587064381304666, 10613932639843599113, 3068375168737395728, 16034891353687438002, 531820664601440079],
[3127187458485921516, 9242108108995753686, 16572214978391878439, 2002264493282317004, 17528372413999311203, 9660133618296276318, 9323255678350276665, 11975985444378500096, 684716641352606611, 5934615398401555169, 3206391255915634099, 5243249714559963838],
[6954578007015926367, 13311604580511007571, 11846386749449887476, 5912005339982720862, 16777881268454767171, 9685147389652966951, 15693739791314941666, 5766626212593531331, 1043928282664672764, 10894151741307035312, 2758986360723730738, 10967594100070378690],
[9709750085078047483, 2955322952929492294, 6344115164294855427, 3953336569737521386, 11901769576745958970, 5206303365926334052, 2966970232915991990, 716781748677738769, 4227277428835128336, 14189688503573647224, 14488582681986863497, 5618800091321945592],
[7679856930326300858, 339747749213877375, 3396440265541284155, 9097427924426491547, 17854127782169052351, 11166339590117763926, 15865265091779522105, 9523738033172770292, 18316940217301626803, 873890966501387965, 11085835142458038285, 14692764916467410279],
[13890312474053084755, 11790296899164099827, 8320517857847627145, 6796012332988113635, 12744825144621584518, 8164068002552027342, 8464535279923249767, 4470451490770703614, 14822503758011380845, 11847511261705382352, 14551798552124347185, 902322957513630679],
[14414665928830143571, 7100752175503142360, 5623668784554864674, 9926346200969810617, 9587330453723752964, 17781593610456656133, 7337546691328150584, 9229589050233377379, 9339560759169708570, 6162634531098054453, 5886346040031932288, 4380524790877007493],
[6494663776345763262, 1361770360025532384, 7510881004738910639, 1250499607479062269, 13044895328286872227, 1410635326646259101, 14234408745338396127, 4822384234879361719, 5703026618965767547, 16275601829595363477, 18249738367837057363, 5842459091888729440],
[12443413895450445295, 9647254829191559590, 8739839481008996472, 4627417554734900085, 969570079706940041, 1547614225577476354, 16582195476354158260, 14928634941823024135, 2558766880990282529, 14146309276987387079, 7235665688510915251, 12326320825232711062],
[18271810496195521614, 2367279045096468937, 16825143120725770692, 12753415863552923177, 11525359558283174243, 1466270998090679878, 6913771438004959295, 7140417199958721368, 15308241555829865598, 11977626414190793852, 12410181912461308815, 6134045924223706650],
[11973163977405796479, 3735952549266839520, 552749508306150531, 15709894569765676041, 15048244651897688193, 9755674922653162582, 12818608036632265137, 7562504519167900152, 9277933304368837129, 3954513067064987083, 7776426702062633097, 5831302322613506931],
[2436167017960685887, 17950961415758255547, 773553818039711851, 2611680250448196130, 17246000505441662136, 2067582258102933435, 12457637224853448133, 12926125228530935463, 7944134927886802790, 8150855496691024125, 16666590939722437137, 1646535586198902855],
[13553072619424034621, 10678087970155447423, 16193879328118936781, 13737333130190187233, 4963477011944736742, 880984241033289885, 427646070753257000, 4675152341835288698, 11984865186156393146, 17386228357720984697, 8201544871501763696, 12178288028704511576],
[6797420598397142817, 6622393268979029263, 9777519620310733322, 12208148426451403893, 12163108832204344910, 3734717762654016843, 15776496231225930394, 6448451711482491464, 12184269664829526930, 13313176558656029974, 16792832898498033114, 7438437987508062815],
[3649630820390029129, 15521640783864839389, 8737928625757462929, 3043387837201416497, 7196837858482925727, 12825672300886618387, 11761706850280417696, 12034783711772781768, 7547274095169916551, 1607614293274501559, 6413611251937212402, 6821257275468811946],
[11151466210328372872, 15015036622830942382, 5268966897012570841, 18054984967494885743, 5330402291063150782, 13456087017270637016, 14381402303158541857, 11477569511287270132, 17734769973454135132, 3306891729381191945, 9468813603273223752, 13274374210438171994],
[17430702662474153728, 1957568024760097144, 948494794092878296, 1362139887516609546, 3649640852289549659, 1154143019582209666, 13832930716471243802, 4558844062449337380, 5276744386718885364, 5998814698887280398, 8479247269974276280, 10129383705858757470],
[14694780564425783680, 10990418703157564008, 5099976139729145180, 16145487508558122191, 336241990480194352, 2883520372435265451, 18048211720587288306, 1014593922571621432, 1897723906411433165, 15287361876763467865, 353593583727002961, 3832452346301167101],
[1695664207309368028, 18253637929532231035, 12532977374309500557, 16106703386580592308, 8209287867132581371, 15569620647379901303, 10236774305760382287, 9257381462225631811, 17171296970224779967, 15658173490059722858, 12120373804335742449, 13449547906661218983],
[1243845830007025470, 12216368958521394628, 17412269497390193377, 7863016412282741846, 15227691571381707347, 12306402470328028596, 6696357235204254483, 2715204407085340607, 4696485459790277165, 18298201355456164231, 13424850082510371158, 8106272284859632876],
[9143711827076790907, 4973596527496026305, 8291789986291197686, 7777616695103516127, 2992831010041911645, 5243732896932325857, 7581515106059941426, 6228359116814322183, 16153866129283884271, 5298096054341997574, 5094560943143533975, 1686117420578331170],
[12900365302243737955, 9688122367774451963, 12470452971963369255, 13236997316376949988, 10736002516973813689, 4409382064652043648, 11938525977209868090, 6299841898815391860, 14040545768211098650, 12680365825250119807, 4114697810609049072, 13309041338056003930],
[34257339389524633, 4755041096540894696, 1270637150104857375, 13321828445917472771, 15615125976951174858, 5530215813703612831, 1549721911473004001, 4651003764819188693, 9266972121914674206, 16697077026589370814, 7159569453302909158, 1545080512034791172],
[12971844614405278793, 17427432586935201905, 14967963749442819583, 8766508405848112988, 16644546236318888584, 11798970864489912668, 4853185816774892110, 15753101789183039017, 11360471177069458808, 4630133945438748411, 5329553070910476857, 4322837256008486795],
[17687780843653045451, 5859480689569342750, 16399637689268344067, 11281092317957420981, 12004746143987929912, 9039101131079296296, 18050303123930927946, 7485267444077923934, 7412268826863387089, 2765404345993399756, 3586650074343563220, 7713305750704015077],
[452723905052761537, 1554534145319595072, 14306082199239566931, 6707596385344934159, 17022616788251914509, 13215082151407790480, 3760094611438789173, 11842096929992248944, 16025304198758035351, 2409465853504599901, 11102033258129177772, 13629612132689840455],
[9044004142118957636, 12127233517740428392, 17342160245270818842, 17250176796685320946, 16591395531375336051, 13954780172636145542, 15598706730880554249, 3717821954965363087, 12698606813635302197, 4502445912403578600, 5758362319240040963, 16778744218263140081],
[2665686981513730504, 15344938367664663447, 5805032920112100559, 14456798373759690426, 3826888120742576315, 14172161278766343193, 16423366163223261022, 385753308002451097, 11312242080452854152, 15553826185710300139, 15155783689913262883, 3790337915931954493]];
