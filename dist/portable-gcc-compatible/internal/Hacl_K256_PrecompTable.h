/* MIT License
 *
 * Copyright (c) 2016-2022 INRIA, CMU and Microsoft Corporation
 * Copyright (c) 2022-2023 HACL* Contributors
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */


#ifndef __internal_Hacl_K256_PrecompTable_H
#define __internal_Hacl_K256_PrecompTable_H

#if defined(__cplusplus)
extern "C" {
#endif

/* SNIPPET_START: Hacl_K256_PrecompTable_precomp_basepoint_table_w4 */

static const
uint64_t
Hacl_K256_PrecompTable_precomp_basepoint_table_w4[240U] =
  {
    0ULL, 0ULL, 0ULL, 0ULL, 0ULL, 1ULL, 0ULL, 0ULL, 0ULL, 0ULL, 0ULL, 0ULL, 0ULL, 0ULL, 0ULL,
    705178180786072ULL, 3855836460717471ULL, 4089131105950716ULL, 3301581525494108ULL,
    133858670344668ULL, 2199641648059576ULL, 1278080618437060ULL, 3959378566518708ULL,
    3455034269351872ULL, 79417610544803ULL, 1ULL, 0ULL, 0ULL, 0ULL, 0ULL, 1282049064345544ULL,
    971732600440099ULL, 1014594595727339ULL, 4392159187541980ULL, 268327875692285ULL,
    2411661712280539ULL, 1092576199280126ULL, 4328619610718051ULL, 3535440816471627ULL,
    95182251488556ULL, 1893725512243753ULL, 3619861457111820ULL, 879374960417905ULL,
    2868056058129113ULL, 273195291893682ULL, 2044797305960112ULL, 2357106853933780ULL,
    3563112438336058ULL, 2430811541762558ULL, 106443809495428ULL, 2231357633909668ULL,
    3641705835951936ULL, 80642569314189ULL, 2254841882373268ULL, 149848031966573ULL,
    2304615661367764ULL, 2410957403736446ULL, 2712754805859804ULL, 2440183877540536ULL,
    99784623895865ULL, 3667773127482758ULL, 1354899394473308ULL, 3636602998800808ULL,
    2709296679846364ULL, 7253362091963ULL, 3585950735562744ULL, 935775991758415ULL,
    4108078106735201ULL, 556081800336307ULL, 229585977163057ULL, 4055594186679801ULL,
    1767681004944933ULL, 1432634922083242ULL, 534935602949197ULL, 251753159522567ULL,
    2846474078499321ULL, 4488649590348702ULL, 2437476916025038ULL, 3040577412822874ULL,
    79405234918614ULL, 3030621226551508ULL, 2801117003929806ULL, 1642927515498422ULL,
    2802725079726297ULL, 8472780626107ULL, 866068070352655ULL, 188080768545106ULL,
    2152119998903058ULL, 3391239985029665ULL, 23820026013564ULL, 2965064154891949ULL,
    1846516097921398ULL, 4418379948133146ULL, 3137755426942400ULL, 47705291301781ULL,
    4278533051105665ULL, 3453643211214931ULL, 3379734319145156ULL, 3762442192097039ULL,
    40243003528694ULL, 4063448994211201ULL, 5697015368785ULL, 1006545411838613ULL,
    4242291693755210ULL, 135184629190512ULL, 264898689131035ULL, 611796474823597ULL,
    3255382250029089ULL, 3490429246984696ULL, 236558595864362ULL, 2055934691551704ULL,
    1487711670114502ULL, 1823930698221632ULL, 2130937287438472ULL, 154610053389779ULL,
    2746573287023216ULL, 2430987262221221ULL, 1668741642878689ULL, 904982541243977ULL,
    56087343124948ULL, 393905062353536ULL, 412681877350188ULL, 3153602040979977ULL,
    4466820876224989ULL, 146579165617857ULL, 2628741216508991ULL, 747994231529806ULL,
    750506569317681ULL, 1887492790748779ULL, 35259008682771ULL, 2085116434894208ULL,
    543291398921711ULL, 1144362007901552ULL, 679305136036846ULL, 141090902244489ULL,
    632480954474859ULL, 2384513102652591ULL, 2225529790159790ULL, 692258664851625ULL,
    198681843567699ULL, 2397092587228181ULL, 145862822166614ULL, 196976540479452ULL,
    3321831130141455ULL, 69266673089832ULL, 4469644227342284ULL, 3899271145504796ULL,
    1261890974076660ULL, 525357673886694ULL, 182135997828583ULL, 4292760618810332ULL,
    3404186545541683ULL, 312297386688768ULL, 204377466824608ULL, 230900767857952ULL,
    3871485172339693ULL, 779449329662955ULL, 978655822464694ULL, 2278252139594027ULL,
    104641527040382ULL, 3528840153625765ULL, 4484699080275273ULL, 1463971951102316ULL,
    4013910812844749ULL, 228915589433620ULL, 1209641433482461ULL, 4043178788774759ULL,
    3008668238856634ULL, 1448425089071412ULL, 26269719725037ULL, 3330785027545223ULL,
    852657975349259ULL, 227245054466105ULL, 1534632353984777ULL, 207715098574660ULL,
    3209837527352280ULL, 4051688046309066ULL, 3839009590725955ULL, 1321506437398842ULL,
    68340219159928ULL, 1806950276956275ULL, 3923908055275295ULL, 743963253393575ULL,
    42162407478783ULL, 261334584474610ULL, 3728224928885214ULL, 4004701081842869ULL,
    709043201644674ULL, 4267294249150171ULL, 255540582975025ULL, 875490593722211ULL,
    796393708218375ULL, 14774425627956ULL, 1500040516752097ULL, 141076627721678ULL,
    2634539368480628ULL, 1106488853550103ULL, 2346231921151930ULL, 897108283954283ULL,
    64616679559843ULL, 400244949840943ULL, 1731263826831733ULL, 1649996579904651ULL,
    3643693449640761ULL, 172543068638991ULL, 329537981097182ULL, 2029799860802869ULL,
    4377737515208862ULL, 29103311051334ULL, 265583594111499ULL, 3798074876561255ULL,
    184749333259352ULL, 3117395073661801ULL, 3695784565008833ULL, 64282709896721ULL,
    1618968913246422ULL, 3185235128095257ULL, 3288745068118692ULL, 1963818603508782ULL,
    281054350739495ULL, 1658639050810346ULL, 3061097601679552ULL, 3023781433263746ULL,
    2770283391242475ULL, 144508864751908ULL, 173576288079856ULL, 46114579547054ULL,
    1679480127300211ULL, 1683062051644007ULL, 117183826129323ULL, 1894068608117440ULL,
    3846899838975733ULL, 4289279019496192ULL, 176995887914031ULL, 78074942938713ULL,
    454207263265292ULL, 972683614054061ULL, 808474205144361ULL, 942703935951735ULL,
    134460241077887ULL
  };

/* SNIPPET_END: Hacl_K256_PrecompTable_precomp_basepoint_table_w4 */

/* SNIPPET_START: Hacl_K256_PrecompTable_precomp_g_pow2_64_table_w4 */

static const
uint64_t
Hacl_K256_PrecompTable_precomp_g_pow2_64_table_w4[240U] =
  {
    0ULL, 0ULL, 0ULL, 0ULL, 0ULL, 1ULL, 0ULL, 0ULL, 0ULL, 0ULL, 0ULL, 0ULL, 0ULL, 0ULL, 0ULL,
    4496295042185355ULL, 3125448202219451ULL, 1239608518490046ULL, 2687445637493112ULL,
    77979604880139ULL, 3360310474215011ULL, 1216410458165163ULL, 177901593587973ULL,
    3209978938104985ULL, 118285133003718ULL, 434519962075150ULL, 1114612377498854ULL,
    3488596944003813ULL, 450716531072892ULL, 66044973203836ULL, 2822827191156652ULL,
    2417714248626059ULL, 2173117567943ULL, 961513119252459ULL, 233852556538333ULL,
    3014783730323962ULL, 2955192634004574ULL, 580546524951282ULL, 2982973948711252ULL,
    226295722018730ULL, 26457116218543ULL, 3401523493637663ULL, 2597746825024790ULL,
    1789211180483113ULL, 155862365823427ULL, 4056806876632134ULL, 1742291745730568ULL,
    3527759000626890ULL, 3740578471192596ULL, 177295097700537ULL, 1533961415657770ULL,
    4305228982382487ULL, 4069090871282711ULL, 4090877481646667ULL, 220939617041498ULL,
    2057548127959588ULL, 45185623103252ULL, 2871963270423449ULL, 3312974792248749ULL,
    8710601879528ULL, 570612225194540ULL, 2045632925323972ULL, 1263913878297555ULL,
    1294592284757719ULL, 238067747295054ULL, 1576659948829386ULL, 2315159636629917ULL,
    3624867787891655ULL, 647628266663887ULL, 75788399640253ULL, 710811707847797ULL,
    130020650130128ULL, 1975045425972589ULL, 136351545314094ULL, 229292031212337ULL,
    1061471455264148ULL, 3281312694184822ULL, 1692442293921797ULL, 4171008525509513ULL,
    275424696197549ULL, 1170296303921965ULL, 4154092952807735ULL, 4371262070870741ULL,
    835769811036496ULL, 275812646528189ULL, 4006745785521764ULL, 1965172239781114ULL,
    4121055644916429ULL, 3578995380229569ULL, 169798870760022ULL, 1834234783016431ULL,
    3186919121688538ULL, 1894269993170652ULL, 868603832348691ULL, 110978471368876ULL,
    1659296605881532ULL, 3257830829309297ULL, 3381509832701119ULL, 4016163121121296ULL,
    265240263496294ULL, 4411285343933251ULL, 728746770806400ULL, 1767819098558739ULL,
    3002081480892841ULL, 96312133241935ULL, 468184501392107ULL, 2061529496271208ULL,
    801565111628867ULL, 3380678576799273ULL, 121814978170941ULL, 3340363319165433ULL,
    2764604325746928ULL, 4475755976431968ULL, 3678073419927081ULL, 237001357924061ULL,
    4110487014553450ULL, 442517757833404ULL, 3976758767423859ULL, 2559863799262476ULL,
    178144664279213ULL, 2488702171798051ULL, 4292079598620208ULL, 1642918280217329ULL,
    3694920319798108ULL, 111735528281657ULL, 2904433967156033ULL, 4391518032143166ULL,
    3018885875516259ULL, 3730342681447122ULL, 10320273322750ULL, 555845881555519ULL,
    58355404017985ULL, 379009359053696ULL, 450317203955503ULL, 271063299686173ULL,
    910340241794202ULL, 4145234574853890ULL, 2059755654702755ULL, 626530377112246ULL,
    188918989156857ULL, 3316657461542117ULL, 778033563170765ULL, 3568562306532187ULL,
    2888619469733481ULL, 4364919962337ULL, 4095057288587059ULL, 2275461355379988ULL,
    1507422995910897ULL, 3737691697116252ULL, 28779913258578ULL, 131453301647952ULL,
    3613515597508469ULL, 2389606941441321ULL, 2135459302594806ULL, 105517262484263ULL,
    2973432939331401ULL, 3447096622477885ULL, 684654106536844ULL, 2815198316729695ULL,
    280303067216071ULL, 1841014812927024ULL, 1181026273060917ULL, 4092989148457730ULL,
    1381045116206278ULL, 112475725893965ULL, 2309144740156686ULL, 1558825847609352ULL,
    2008068002046292ULL, 3153511625856423ULL, 38469701427673ULL, 4240572315518056ULL,
    2295170987320580ULL, 187734093837094ULL, 301041528077172ULL, 234553141005715ULL,
    4170513699279606ULL, 1600132848196146ULL, 3149113064155689ULL, 2733255352600949ULL,
    144915931419495ULL, 1221012073888926ULL, 4395668111081710ULL, 2464799161496070ULL,
    3664256125241313ULL, 239705368981290ULL, 1415181408539490ULL, 2551836620449074ULL,
    3003106895689578ULL, 968947218886924ULL, 270781532362673ULL, 2905980714350372ULL,
    3246927349288975ULL, 2653377642686974ULL, 1577457093418263ULL, 279488238785848ULL,
    568335962564552ULL, 4251365041645758ULL, 1257832559776007ULL, 2424022444243863ULL,
    261166122046343ULL, 4399874608082116ULL, 640509987891568ULL, 3119706885332220ULL,
    1990185416694007ULL, 119390098529341ULL, 220106534694050ULL, 937225880034895ULL,
    656288151358882ULL, 1766967254772100ULL, 197900790969750ULL, 2992539221608875ULL,
    3960297171111858ULL, 3499202002925081ULL, 1103060980924705ULL, 13670895919578ULL,
    430132744187721ULL, 1206771838050953ULL, 2474749300167198ULL, 296299539510780ULL,
    61565517686436ULL, 752778559080573ULL, 3049015829565410ULL, 3538647632527371ULL,
    1640473028662032ULL, 182488721849306ULL, 1234378482161516ULL, 3736205988606381ULL,
    2814216844344487ULL, 3877249891529557ULL, 51681412928433ULL, 4275336620301239ULL,
    3084074032750651ULL, 42732308350456ULL, 3648603591552229ULL, 142450621701603ULL,
    4020045475009854ULL, 1050293952073054ULL, 1974773673079851ULL, 1815515638724020ULL,
    104845375825434ULL
  };

/* SNIPPET_END: Hacl_K256_PrecompTable_precomp_g_pow2_64_table_w4 */

/* SNIPPET_START: Hacl_K256_PrecompTable_precomp_g_pow2_128_table_w4 */

static const
uint64_t
Hacl_K256_PrecompTable_precomp_g_pow2_128_table_w4[240U] =
  {
    0ULL, 0ULL, 0ULL, 0ULL, 0ULL, 1ULL, 0ULL, 0ULL, 0ULL, 0ULL, 0ULL, 0ULL, 0ULL, 0ULL, 0ULL,
    1277614565900951ULL, 378671684419493ULL, 3176260448102880ULL, 1575691435565077ULL,
    167304528382180ULL, 2600787765776588ULL, 7497946149293ULL, 2184272641272202ULL,
    2200235265236628ULL, 265969268774814ULL, 1913228635640715ULL, 2831959046949342ULL,
    888030405442963ULL, 1817092932985033ULL, 101515844997121ULL, 3309468394859588ULL,
    3965334773689948ULL, 1945272965790738ULL, 4450939211427964ULL, 211349698782702ULL,
    2085160302160079ULL, 212812506072603ULL, 3646122434511764ULL, 1711405092320514ULL,
    95160920508464ULL, 1677683368518073ULL, 4384656939250953ULL, 3548591046529893ULL,
    1683233536091384ULL, 105919586159941ULL, 1941416002726455ULL, 246264372248216ULL,
    3063044110922228ULL, 3772292170415825ULL, 222933374989815ULL, 2417211163452935ULL,
    2018230365573200ULL, 1985974538911047ULL, 1387197705332739ULL, 186400825584956ULL,
    2469330487750329ULL, 1291983813301638ULL, 333416733706302ULL, 3413315564261070ULL,
    189444777569683ULL, 1062005622360420ULL, 1800197715938740ULL, 3693110992551647ULL,
    626990328941945ULL, 40998857100520ULL, 3921983552805085ULL, 1016632437340656ULL,
    4016615929950878ULL, 2682554586771281ULL, 7043555162389ULL, 3333819830676567ULL,
    4120091964944036ULL, 1960788263484015ULL, 1642145656273304ULL, 252814075789128ULL,
    3085777342821357ULL, 4166637997604052ULL, 1339401689756469ULL, 845938529607551ULL,
    223351828189283ULL, 1148648705186890ULL, 1230525014760605ULL, 1869739475126720ULL,
    4193966261205530ULL, 175684010336013ULL, 4476719358931508ULL, 4209547487457638ULL,
    2197536411673724ULL, 3010838433412303ULL, 169318997251483ULL, 49493868302162ULL,
    3594601099078584ULL, 3662420905445942ULL, 3606544932233685ULL, 270643652662165ULL,
    180681786228544ULL, 2095882682308564ULL, 813484483841391ULL, 1622665392824698ULL,
    113821770225137ULL, 3075432444115417ULL, 716502989978722ULL, 2304779892217245ULL,
    1760144151770127ULL, 235719156963938ULL, 3180013070471143ULL, 1331027634540579ULL,
    552273022992392ULL, 2858693077461887ULL, 197914407731510ULL, 187252310910959ULL,
    4160637171377125ULL, 3225059526713298ULL, 2574558217383978ULL, 249695600622489ULL,
    364988742814327ULL, 4245298536326258ULL, 1812464706589342ULL, 2734857123772998ULL,
    120105577124628ULL, 160179251271109ULL, 3604555733307834ULL, 150380003195715ULL,
    1574304909935121ULL, 142190285600761ULL, 1835385847725651ULL, 3168087139615901ULL,
    3201434861713736ULL, 741757984537760ULL, 163585009419543ULL, 3837997981109783ULL,
    3771946407870997ULL, 2867641360295452ULL, 3097548691501578ULL, 124624912142104ULL,
    2729896088769328ULL, 1087786827035225ULL, 3934000813818614ULL, 1176792318645055ULL,
    125311882169270ULL, 3530709439299502ULL, 1561477829834527ULL, 3927894570196761ULL,
    3957765307669212ULL, 105720519513730ULL, 3758969845816997ULL, 2738320452287300ULL,
    2380753632109507ULL, 2762090901149075ULL, 123455059136515ULL, 4222807813169807ULL,
    118064783651432ULL, 2877694712254934ULL, 3535027426396448ULL, 100175663703417ULL,
    3287921121213155ULL, 4497246481824206ULL, 1960809949007025ULL, 3236854264159102ULL,
    35028112623717ULL, 338838627913273ULL, 2827531947914645ULL, 4231826783810670ULL,
    1082490106100389ULL, 13267544387448ULL, 4249975884259105ULL, 2844862161652484ULL,
    262742197948971ULL, 3525653802457116ULL, 269963889261701ULL, 3690062482117102ULL,
    675413453822147ULL, 2170937868437574ULL, 2367632187022010ULL, 214032802409445ULL,
    2054007379612477ULL, 3558050826739009ULL, 266827184752634ULL, 1946520293291195ULL,
    238087872386556ULL, 490056555385700ULL, 794405769357386ULL, 3886901294859702ULL,
    3120414548626348ULL, 84316625221136ULL, 223073962531835ULL, 4280846460577631ULL,
    344296282849308ULL, 3522116652699457ULL, 171817232053075ULL, 3296636283062273ULL,
    3587303364425579ULL, 1033485783633331ULL, 3686984130812906ULL, 268290803650477ULL,
    2803988215834467ULL, 3821246410529720ULL, 1077722388925870ULL, 4187137036866164ULL,
    104696540795905ULL, 998770003854764ULL, 3960768137535019ULL, 4293792474919135ULL,
    3251297981727034ULL, 192479028790101ULL, 1175880869349935ULL, 3506949259311937ULL,
    2161711516160714ULL, 2506820922270187ULL, 131002200661047ULL, 3532399477339994ULL,
    2515815721228719ULL, 4274974119021502ULL, 265752394510924ULL, 163144272153395ULL,
    2824260010502991ULL, 517077012665142ULL, 602987073882924ULL, 2939630061751780ULL,
    59211609557440ULL, 963423614549333ULL, 495476232754434ULL, 94274496109103ULL,
    2245136222990187ULL, 185414764872288ULL, 2266067668609289ULL, 3873978896235927ULL,
    4428283513152105ULL, 3881481480259312ULL, 207746202010862ULL, 1609437858011364ULL,
    477585758421515ULL, 3850430788664649ULL, 2682299074459173ULL, 149439089751274ULL,
    3665760243877698ULL, 1356661512658931ULL, 1675903262368322ULL, 3355649228050892ULL,
    99772108898412ULL
  };

/* SNIPPET_END: Hacl_K256_PrecompTable_precomp_g_pow2_128_table_w4 */

/* SNIPPET_START: Hacl_K256_PrecompTable_precomp_g_pow2_192_table_w4 */

static const
uint64_t
Hacl_K256_PrecompTable_precomp_g_pow2_192_table_w4[240U] =
  {
    0ULL, 0ULL, 0ULL, 0ULL, 0ULL, 1ULL, 0ULL, 0ULL, 0ULL, 0ULL, 0ULL, 0ULL, 0ULL, 0ULL, 0ULL,
    34056422761564ULL, 3315864838337811ULL, 3797032336888745ULL, 2580641850480806ULL,
    208048944042500ULL, 1233795288689421ULL, 1048795233382631ULL, 646545158071530ULL,
    1816025742137285ULL, 12245672982162ULL, 2119364213800870ULL, 2034960311715107ULL,
    3172697815804487ULL, 4185144850224160ULL, 2792055915674ULL, 795534452139321ULL,
    3647836177838185ULL, 2681403398797991ULL, 3149264270306207ULL, 278704080615511ULL,
    2752552368344718ULL, 1363840972378818ULL, 1877521512083293ULL, 1862111388059470ULL,
    36200324115014ULL, 4183622899327217ULL, 747381675363076ULL, 2772916395314624ULL,
    833767013119965ULL, 246274452928088ULL, 1526238021297781ULL, 3327534966022747ULL,
    1169012581910517ULL, 4430894603030025ULL, 149242742442115ULL, 1002569704307172ULL,
    2763252093432365ULL, 3037748497732938ULL, 2329811173939457ULL, 270769113180752ULL,
    4344092461623432ULL, 892200524589382ULL, 2511418516713970ULL, 103575031265398ULL,
    183736033430252ULL, 583003071257308ULL, 3357167344738425ULL, 4038099763242651ULL,
    1776250620957255ULL, 51334115864192ULL, 2616405698969611ULL, 1196364755910565ULL,
    3135228056210500ULL, 533729417611761ULL, 86564351229326ULL, 98936129527281ULL,
    4425305036630677ULL, 2980296390253408ULL, 2487091677325739ULL, 10501977234280ULL,
    1805646499831077ULL, 3120615962395477ULL, 3634629685307533ULL, 3009632755291436ULL,
    16794051906523ULL, 2465481597883214ULL, 211492787490403ULL, 1120942867046103ULL,
    486438308572108ULL, 76058986271771ULL, 2435216584587357ULL, 3076359381968283ULL,
    1071594491489655ULL, 3148707450339154ULL, 249332205737851ULL, 4171051176626809ULL,
    3165176227956388ULL, 2400901591835233ULL, 1435783621333022ULL, 20312753440321ULL,
    1767293887448005ULL, 685150647587522ULL, 2957187934449906ULL, 382661319140439ULL,
    177583591139601ULL, 2083572648630743ULL, 1083410277889419ULL, 4267902097868310ULL,
    679989918385081ULL, 123155311554032ULL, 2830267662472020ULL, 4476040509735924ULL,
    526697201585144ULL, 3465306430573135ULL, 2296616218591ULL, 1270626872734279ULL,
    1049740198790549ULL, 4197567214843444ULL, 1962225231320591ULL, 186125026796856ULL,
    737027567341142ULL, 4364616098174ULL, 3618884818756660ULL, 1236837563717668ULL,
    162873772439548ULL, 3081542470065122ULL, 910331750163991ULL, 2110498143869827ULL,
    3208473121852657ULL, 94687786224509ULL, 4113309027567819ULL, 4272179438357536ULL,
    1857418654076140ULL, 1672678841741004ULL, 94482160248411ULL, 1928652436799020ULL,
    1750866462381515ULL, 4048060485672270ULL, 4006680581258587ULL, 14850434761312ULL,
    2828734997081648ULL, 1975589525873972ULL, 3724347738416009ULL, 597163266689736ULL,
    14568362978551ULL, 2203865455839744ULL, 2237034958890595ULL, 1863572986731818ULL,
    2329774560279041ULL, 245105447642201ULL, 2179697447864822ULL, 1769609498189882ULL,
    1916950746430931ULL, 847019613787312ULL, 163210606565100ULL, 3658248417400062ULL,
    717138296045881ULL, 42531212306121ULL, 1040915917097532ULL, 77364489101310ULL,
    539253504015590ULL, 732690726289841ULL, 3401622034697806ULL, 2864593278358513ULL,
    142611941887017ULL, 536364617506702ULL, 845071859974284ULL, 4461787417089721ULL,
    2633811871939723ULL, 113619731985610ULL, 2535870015489566ULL, 2146224665077830ULL,
    2593725534662047ULL, 1332349537449710ULL, 153375287068096ULL, 3689977177165276ULL,
    3631865615314120ULL, 184644878348929ULL, 2220481726602813ULL, 204002551273091ULL,
    3022560051766785ULL, 3125940458001213ULL, 4258299086906325ULL, 1072471915162030ULL,
    2797562724530ULL, 3974298156223059ULL, 1624778551002554ULL, 3490703864485971ULL,
    2533877484212458ULL, 176107782538555ULL, 4275987398312137ULL, 4397120757693722ULL,
    3001292763847390ULL, 1556490837621310ULL, 70442953037671ULL, 1558915972545974ULL,
    744724505252845ULL, 2697230204313363ULL, 3495671924212144ULL, 95744296878924ULL,
    1508848630912047ULL, 4163599342850968ULL, 1234988733935901ULL, 3789722472212706ULL,
    219522007052022ULL, 2106597506701262ULL, 3231115099832239ULL, 1296436890593905ULL,
    1016795619587656ULL, 231150565033388ULL, 4205501688458754ULL, 2271569140386062ULL,
    3421769599058157ULL, 4118408853784554ULL, 276709341465173ULL, 2681340614854362ULL,
    2514413365628788ULL, 62294545067341ULL, 277610220069365ULL, 252463150123799ULL,
    2547353593759399ULL, 1857438147448607ULL, 2964811969681256ULL, 3303706463835387ULL,
    248936570980853ULL, 3208982702478009ULL, 2518671051730787ULL, 727433853033835ULL,
    1290389308223446ULL, 220742793981035ULL, 3851225361654709ULL, 2307489307934273ULL,
    1151710489948266ULL, 289775285210516ULL, 222685002397295ULL, 1222117478082108ULL,
    2822029169395728ULL, 1172146252219882ULL, 2626108105510259ULL, 209803527887167ULL,
    2718831919953281ULL, 4348638387588593ULL, 3761438313263183ULL, 13169515318095ULL,
    212893621229476ULL
  };

/* SNIPPET_END: Hacl_K256_PrecompTable_precomp_g_pow2_192_table_w4 */

/* SNIPPET_START: Hacl_K256_PrecompTable_precomp_basepoint_table_w5 */

static const
uint64_t
Hacl_K256_PrecompTable_precomp_basepoint_table_w5[480U] =
  {
    0ULL, 0ULL, 0ULL, 0ULL, 0ULL, 1ULL, 0ULL, 0ULL, 0ULL, 0ULL, 0ULL, 0ULL, 0ULL, 0ULL, 0ULL,
    705178180786072ULL, 3855836460717471ULL, 4089131105950716ULL, 3301581525494108ULL,
    133858670344668ULL, 2199641648059576ULL, 1278080618437060ULL, 3959378566518708ULL,
    3455034269351872ULL, 79417610544803ULL, 1ULL, 0ULL, 0ULL, 0ULL, 0ULL, 1282049064345544ULL,
    971732600440099ULL, 1014594595727339ULL, 4392159187541980ULL, 268327875692285ULL,
    2411661712280539ULL, 1092576199280126ULL, 4328619610718051ULL, 3535440816471627ULL,
    95182251488556ULL, 1893725512243753ULL, 3619861457111820ULL, 879374960417905ULL,
    2868056058129113ULL, 273195291893682ULL, 2044797305960112ULL, 2357106853933780ULL,
    3563112438336058ULL, 2430811541762558ULL, 106443809495428ULL, 2231357633909668ULL,
    3641705835951936ULL, 80642569314189ULL, 2254841882373268ULL, 149848031966573ULL,
    2304615661367764ULL, 2410957403736446ULL, 2712754805859804ULL, 2440183877540536ULL,
    99784623895865ULL, 3667773127482758ULL, 1354899394473308ULL, 3636602998800808ULL,
    2709296679846364ULL, 7253362091963ULL, 3585950735562744ULL, 935775991758415ULL,
    4108078106735201ULL, 556081800336307ULL, 229585977163057ULL, 4055594186679801ULL,
    1767681004944933ULL, 1432634922083242ULL, 534935602949197ULL, 251753159522567ULL,
    2846474078499321ULL, 4488649590348702ULL, 2437476916025038ULL, 3040577412822874ULL,
    79405234918614ULL, 3030621226551508ULL, 2801117003929806ULL, 1642927515498422ULL,
    2802725079726297ULL, 8472780626107ULL, 866068070352655ULL, 188080768545106ULL,
    2152119998903058ULL, 3391239985029665ULL, 23820026013564ULL, 2965064154891949ULL,
    1846516097921398ULL, 4418379948133146ULL, 3137755426942400ULL, 47705291301781ULL,
    4278533051105665ULL, 3453643211214931ULL, 3379734319145156ULL, 3762442192097039ULL,
    40243003528694ULL, 4063448994211201ULL, 5697015368785ULL, 1006545411838613ULL,
    4242291693755210ULL, 135184629190512ULL, 264898689131035ULL, 611796474823597ULL,
    3255382250029089ULL, 3490429246984696ULL, 236558595864362ULL, 2055934691551704ULL,
    1487711670114502ULL, 1823930698221632ULL, 2130937287438472ULL, 154610053389779ULL,
    2746573287023216ULL, 2430987262221221ULL, 1668741642878689ULL, 904982541243977ULL,
    56087343124948ULL, 393905062353536ULL, 412681877350188ULL, 3153602040979977ULL,
    4466820876224989ULL, 146579165617857ULL, 2628741216508991ULL, 747994231529806ULL,
    750506569317681ULL, 1887492790748779ULL, 35259008682771ULL, 2085116434894208ULL,
    543291398921711ULL, 1144362007901552ULL, 679305136036846ULL, 141090902244489ULL,
    632480954474859ULL, 2384513102652591ULL, 2225529790159790ULL, 692258664851625ULL,
    198681843567699ULL, 2397092587228181ULL, 145862822166614ULL, 196976540479452ULL,
    3321831130141455ULL, 69266673089832ULL, 4469644227342284ULL, 3899271145504796ULL,
    1261890974076660ULL, 525357673886694ULL, 182135997828583ULL, 4292760618810332ULL,
    3404186545541683ULL, 312297386688768ULL, 204377466824608ULL, 230900767857952ULL,
    3871485172339693ULL, 779449329662955ULL, 978655822464694ULL, 2278252139594027ULL,
    104641527040382ULL, 3528840153625765ULL, 4484699080275273ULL, 1463971951102316ULL,
    4013910812844749ULL, 228915589433620ULL, 1209641433482461ULL, 4043178788774759ULL,
    3008668238856634ULL, 1448425089071412ULL, 26269719725037ULL, 3330785027545223ULL,
    852657975349259ULL, 227245054466105ULL, 1534632353984777ULL, 207715098574660ULL,
    3209837527352280ULL, 4051688046309066ULL, 3839009590725955ULL, 1321506437398842ULL,
    68340219159928ULL, 1806950276956275ULL, 3923908055275295ULL, 743963253393575ULL,
    42162407478783ULL, 261334584474610ULL, 3728224928885214ULL, 4004701081842869ULL,
    709043201644674ULL, 4267294249150171ULL, 255540582975025ULL, 875490593722211ULL,
    796393708218375ULL, 14774425627956ULL, 1500040516752097ULL, 141076627721678ULL,
    2634539368480628ULL, 1106488853550103ULL, 2346231921151930ULL, 897108283954283ULL,
    64616679559843ULL, 400244949840943ULL, 1731263826831733ULL, 1649996579904651ULL,
    3643693449640761ULL, 172543068638991ULL, 329537981097182ULL, 2029799860802869ULL,
    4377737515208862ULL, 29103311051334ULL, 265583594111499ULL, 3798074876561255ULL,
    184749333259352ULL, 3117395073661801ULL, 3695784565008833ULL, 64282709896721ULL,
    1618968913246422ULL, 3185235128095257ULL, 3288745068118692ULL, 1963818603508782ULL,
    281054350739495ULL, 1658639050810346ULL, 3061097601679552ULL, 3023781433263746ULL,
    2770283391242475ULL, 144508864751908ULL, 173576288079856ULL, 46114579547054ULL,
    1679480127300211ULL, 1683062051644007ULL, 117183826129323ULL, 1894068608117440ULL,
    3846899838975733ULL, 4289279019496192ULL, 176995887914031ULL, 78074942938713ULL,
    454207263265292ULL, 972683614054061ULL, 808474205144361ULL, 942703935951735ULL,
    134460241077887ULL, 2104196179349630ULL, 501632371208418ULL, 1666838991431177ULL,
    445606193139838ULL, 73704603396096ULL, 3140284774064777ULL, 1356066420820179ULL,
    227054159419281ULL, 1847611229198687ULL, 82327838827660ULL, 3704027573265803ULL,
    1585260489220244ULL, 4404647914931933ULL, 2424649827425515ULL, 206821944206116ULL,
    1508635776287972ULL, 1933584575629676ULL, 1903635423783032ULL, 4193642165165650ULL,
    234321074690644ULL, 210406774251925ULL, 1965845668185599ULL, 3059839433804731ULL,
    1933300510683631ULL, 150696600689211ULL, 4069293682158567ULL, 4346344602660044ULL,
    312200249664561ULL, 2495020807621840ULL, 1912707714385ULL, 299345978159762ULL,
    1164752722686920ULL, 225322433710338ULL, 3128747381283759ULL, 275659067815583ULL,
    1489671057429039ULL, 1567693343342676ULL, 921672046098071ULL, 3707418899384085ULL,
    54646424931593ULL, 4026733380127147ULL, 2933435393699231ULL, 3356593659521967ULL,
    3637750749325529ULL, 232939412379045ULL, 2298399636043069ULL, 270361546063041ULL,
    2523933572551420ULL, 3456896091572950ULL, 185447004732850ULL, 429322937697821ULL,
    2579704215668222ULL, 695065378803349ULL, 3987916247731243ULL, 255159546348233ULL,
    3057777929921282ULL, 1608970699916312ULL, 1902369623063807ULL, 1413619643652777ULL,
    94983996321227ULL, 2832873179548050ULL, 4335430233622555ULL, 1559023976028843ULL,
    3297181988648895ULL, 100072021232323ULL, 2124984034109675ULL, 4501252835618918ULL,
    2053336899483297ULL, 638807226463876ULL, 278445213600634ULL, 2311236445660555ULL,
    303317664040012ULL, 2659353858089024ULL, 3598827423980130ULL, 176059343827873ULL,
    3891639526275437ULL, 252823982819463ULL, 3404823300622345ULL, 2758370772497456ULL,
    91397496598783ULL, 2248661144141892ULL, 491087075271969ULL, 1786344894571315ULL,
    452497694885923ULL, 34039628873357ULL, 2116503165025197ULL, 4436733709429923ULL,
    3045800776819238ULL, 1385518906078375ULL, 110495603336764ULL, 4051447296249587ULL,
    1103557421498625ULL, 1840785058439622ULL, 425322753992314ULL, 98330046771676ULL,
    365407468686431ULL, 2611246859977123ULL, 3050253933135339ULL, 1006482220896688ULL,
    166818196428389ULL, 3415236093104372ULL, 1762308883882288ULL, 1327828123094558ULL,
    3403946425556706ULL, 96503464455441ULL, 3893015304031471ULL, 3740839477490397ULL,
    2411470812852231ULL, 940927462436211ULL, 163825285911099ULL, 1622441495640386ULL,
    850224095680266ULL, 76199085900939ULL, 1941852365144042ULL, 140326673652807ULL,
    3161611011249524ULL, 317297150009965ULL, 2145053259340619ULL, 2180498176457552ULL,
    38457740506224ULL, 394174899129468ULL, 2687474560485245ULL, 1542175980184516ULL,
    1628502671124819ULL, 48477401124385ULL, 4474181600025082ULL, 2142747956365708ULL,
    1638299432475478ULL, 2005869320353249ULL, 112292630760956ULL, 1887521965171588ULL,
    457587531429696ULL, 840994209504042ULL, 4268060856325798ULL, 195597993440388ULL,
    4148484749020338ULL, 2074885000909672ULL, 2309839019263165ULL, 2087616209681024ULL,
    257214370719966ULL, 2331363508376581ULL, 1233124357504711ULL, 2849542202650296ULL,
    3790982825325736ULL, 13381453503890ULL, 1665246594531069ULL, 4165624287443904ULL,
    3418759698027493ULL, 2118493255117399ULL, 136249206366067ULL, 4064050233283309ULL,
    1368779887911300ULL, 4370550759530269ULL, 66992990631341ULL, 84442368922270ULL,
    2139322635321394ULL, 2076163483726795ULL, 657097866349103ULL, 2095579409488071ULL,
    226525774791341ULL, 4445744257665359ULL, 2035752839278107ULL, 1998242662838304ULL,
    1601548415521694ULL, 151297684296198ULL, 1350963039017303ULL, 2624916349548281ULL,
    2018863259670197ULL, 2717274357461290ULL, 94024796961533ULL, 711335520409111ULL,
    4322093765820263ULL, 2041650358174649ULL, 3439791603157577ULL, 179292018616267ULL,
    2436436921286669ULL, 3905268797208340ULL, 2829194895162985ULL, 1355175382191543ULL,
    55128779761539ULL, 2648428998786922ULL, 869805912573515ULL, 3706708942847864ULL,
    2785288916584667ULL, 37156862850147ULL, 1422245336293228ULL, 4497066058933021ULL,
    85588912978349ULL, 2616252221194611ULL, 53506393720989ULL, 3727539190732644ULL,
    872132446545237ULL, 933583590986077ULL, 3794591170581203ULL, 167875550514069ULL,
    2267466834993297ULL, 3072652681756816ULL, 2108499037430803ULL, 1606735192928366ULL,
    72339568815255ULL, 3258484260684219ULL, 3277927277719855ULL, 2459560373011535ULL,
    1672794293294033ULL, 227460934880669ULL, 3702454405413705ULL, 106168148441676ULL,
    1356617643071159ULL, 3280896569942762ULL, 142618711614302ULL, 4291782740862057ULL,
    4141020884874235ULL, 3720787221267125ULL, 552884940089351ULL, 174626154407180ULL,
    972071013326540ULL, 4458530419931903ULL, 4435168973822858ULL, 1902967548748411ULL,
    53007977605840ULL, 2453997334323925ULL, 3653077937283262ULL, 850660265046356ULL,
    312721924805450ULL, 268503679240683ULL, 256960167714122ULL, 1474492507858350ULL,
    2456345526438488ULL, 3686029507160255ULL, 279158933010398ULL, 3646946293948063ULL,
    704477527214036ULL, 3387744169891031ULL, 3772622670980241ULL, 136368897543304ULL,
    3744894052577607ULL, 1976007214443430ULL, 2090045379763451ULL, 968565474458988ULL,
    234295114806066ULL
  };

/* SNIPPET_END: Hacl_K256_PrecompTable_precomp_basepoint_table_w5 */

#if defined(__cplusplus)
}
#endif

#define __internal_Hacl_K256_PrecompTable_H_DEFINED
#endif
