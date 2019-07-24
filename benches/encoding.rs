use criterion::{black_box, Criterion};
use marysue::encode;

pub const TEST_21: &str = "苟利国家生死以";
pub const TEST_195: &str = "\
力微任重久神疲，再竭衰庸定不支。
苟利国家生死以，岂因祸福避趋之？
谪居正是君恩厚，养拙刚于戍卒宜。
戏与山妻谈故事，试吟断送老头皮。";
pub const TEST_2909: &str = "\
汉皇重色思倾国，御宇多年求不得。杨家有女初长成，养在深闺人未识。
天生丽质难自弃，一朝选在君王侧。回眸一笑百媚生，六宫粉黛无颜色。
春寒赐浴华清池，温泉水滑洗凝脂。侍儿扶起娇无力，始是新承恩泽时。
云鬓花颜金步摇，芙蓉帐暖度春宵。春宵苦短日高起，从此君王不早朝。
承欢侍宴无闲暇，春从春游夜专夜。后宫佳丽三千人，三千宠爱在一身。
金屋妆成娇侍夜，玉楼宴罢醉和春。姊妹弟兄皆列土，可怜光彩生门户。
遂令天下父母心，不重生男重生女。骊宫高处入青云，仙乐风飘处处闻。
缓歌慢舞凝丝竹，尽日君王看不足。渔阳鼙鼓动地来，惊破霓裳羽衣曲。
九重城阙烟尘生，千乘万骑西南行。翠华摇摇行复止，西出都门百余里。
六军不发无奈何，宛转蛾眉马前死。花钿委地无人收，翠翘金雀玉搔头。
君王掩面救不得，回看血泪相和流。黄埃散漫风萧索，云栈萦纡登剑阁。
峨嵋山下少人行，旌旗无光日色薄。蜀江水碧蜀山青，圣主朝朝暮暮情。
行宫见月伤心色，夜雨闻铃肠断声。天旋日转回龙驭，到此踌躇不能去。
马嵬坡下泥土中，不见玉颜空死处。君臣相顾尽沾衣，东望都门信马归。
归来池苑皆依旧，太液芙蓉未央柳。芙蓉如面柳如眉，对此如何不泪垂？
春风桃李花开夜，秋雨梧桐叶落时。西宫南苑多秋草，落叶满阶红不扫。
梨园弟子白发新，椒房阿监青娥老。夕殿萤飞思悄然，孤灯挑尽未成眠。
迟迟钟鼓初长夜，耿耿星河欲曙天。鸳鸯瓦冷霜华重，翡翠衾寒谁与共？
悠悠生死别经年，魂魄不曾来入梦。临邛道士鸿都客，能以精诚致魂魄。
为感君王辗转思，遂教方士殷勤觅。排空驭气奔如电，升天入地求之遍。
上穷碧落下黄泉，两处茫茫皆不见。忽闻海上有仙山，山在虚无缥缈间。
楼阁玲珑五云起，其中绰约多仙子。中有一人字太真，雪肤花貌参差是。
金阙西厢叩玉扃，转教小玉报双成。闻道汉家天子使，九华帐里梦魂惊。
揽衣推枕起徘徊，珠箔银屏迤逦开。云鬓半偏新睡觉，花冠不整下堂来。
风吹仙袂飘飖举，犹似霓裳羽衣舞。玉容寂寞泪阑干，梨花一枝春带雨。
含情凝睇谢君王，一别音容两渺茫。昭阳殿里恩爱绝，蓬莱宫中日月长。
回头下望人寰处，不见长安见尘雾。惟将旧物表深情，钿合金钗寄将去。
钗留一股合一扇，钗擘黄金合分钿。但令心似金钿坚，天上人间会相见。
临别殷勤重寄词，词中有誓两心知。七月七日长生殿，夜半无人私语时。
在天愿作比翼鸟，在地愿为连理枝。天长地久有时尽，此恨绵绵无绝期。";
pub const TEST_19414: &str = "\
道可道，非常道。名可名，非常名。无名天地之始；有名万物之母。故常无欲，以观其妙；常有欲，以观其徼。此两者，同出而异名，同谓之玄。玄之又玄，衆妙之门。
天下皆知美之为美，斯恶已。皆知善之为善，斯不善已。故有无相生，难易相成，长短相较，高下相倾，音声相和，前后相随。是以圣人处无为之事，行不言之教；万物作焉而不辞，生而不有。为而不恃，功成而弗居。夫唯弗居，是以不去。
不尚贤，使民不争；不贵难得之货，使民不为盗；不见可欲，使心不乱。是以圣人之治，虚其心，实其腹，弱其志，强其骨。常使民无知无欲。使天知者不敢为也。为无为，则无不治。
道冲而用之或不盈。渊兮似万物之宗。挫其锐，解其纷，和其光，同其尘。湛兮似或存。吾不知谁之子，象帝之先。
天地不仁，以万物为刍狗；圣人不仁，以百姓为刍狗。天地之间，其犹橐龠乎？虚而不屈，动而愈出。多言数穷，不如守中。
谷神不死，是谓玄牝。玄牝之门，是谓天地根。绵绵若存，用之不勤。
天长地久。天地所以能长且久者，以其不自生，故能长生。是以圣人后其身而身先；外其身而身存。非以其无私耶？故能成其私。
上善若水。水善利万物而不争，处衆人之所恶，故几于道。居善地，心善渊，与善仁，言善信，正善治，事善能，动善时。夫唯不争，故无尤。
持而盈之，不如其已；揣而锐之，不可长保。金玉满堂，莫之能守；富贵而骄，自遗其咎。功遂身退天之道。
载营魄抱一，能无离乎？专气致柔，能婴儿乎？涤除玄览，能无疵乎？爱民治国，能无知乎？天门开阖，能为雌乎？明白四达，能无知乎？生之、畜之，生而不有，为而不恃，长而不宰，是谓玄德。
三十辐，共一毂，当其无，有车之用。埏埴以为器，当其无，有器之用。凿户牖以为室，当其无，有室之用。故有之以为利，无之以为用。
五色令人目盲；五音令人耳聋；五味令人口爽；驰骋田猎，令人心发狂；难得之货，令人行妨。是以圣人为腹不为目，故去彼取此。
宠辱若惊，贵大患若身。何谓宠辱若惊？宠为下，得之若惊，失之若惊，是谓宠辱若惊。何谓贵大患若身？吾所以有大患者，为吾有身，及吾无身，吾有何患？故贵以身为天下，若可寄天下；爱以身为天下，若可托天下。
视之不见，名曰夷；听之不闻，名曰希；搏之不得，名曰微。此三者不可致诘，故混而为一。其上不皦，其下不昧。绳绳不可名，复归于无物。是谓无状之状，无物之象，是谓惚恍。迎之不见其首，随之不见其后。执古之道，以御今之有。能知古始，是谓道纪。
古之善为士者，微妙玄通，深不可识。夫唯不可识，故强为之容。豫兮若冬涉川；犹兮若畏四邻；俨兮其若容；涣兮若冰之将释；敦兮其若朴；旷兮其若谷；混兮其若浊；孰能浊以静之徐清？孰能安以久动之徐生？保此道者，不欲盈。夫唯不盈，故能蔽不新成。
致虚极，守静笃。万物并作，吾以观复。夫物芸芸，各复归其根。归根曰静，是谓复命。复命曰常，知常曰明。不知常，妄作凶。知常容，容乃公，公乃王，王乃天，天乃道，道乃久，没身不殆。
太上，下知有之；其次，亲而誉之；其次，畏之；其次，侮之。信不足，焉有不信焉。悠兮，其贵言。功成事遂，百姓皆谓我自然。
大道废，有仁义；智慧出，有大伪；六亲不和，有孝慈；国家昏乱，有忠臣。
绝圣弃智，民利百倍；绝仁弃义，民复孝慈；绝巧弃利，盗贼无有。此三者以为文不足。故令有所属：见素抱朴，少私寡欲。
绝学无忧，唯之与阿，相去几何？善之与恶，相去若何？人之所畏，不可不畏。荒兮其未央哉！衆人熙熙，如享太牢，如春登台。我独怕兮其未兆；如婴儿之未孩；儽儽兮若无所归。衆人皆有馀，而我独若遗。我愚人之心也哉！沌沌兮，俗人昭昭，我独若昏。俗人察察，我独闷闷。澹兮其若海，飂兮若无止，衆人皆有以，而我独顽似鄙。我独异于人，而贵食母。
孔德之容，唯道是从。道之为物，唯恍唯惚。忽兮恍兮，其中有象；恍兮忽兮，其中有物。窈兮冥兮，其中有精；其精甚真，其中有信。自古及今，其名不去，以阅衆甫。吾何以知衆甫之状哉？以此。
曲则全，枉则直，洼则盈，弊则新，少则得，多则惑。是以圣人抱一为天下式。不自见，故明；不自是，故彰；不自伐，故有功；不自矜，故长。夫唯不争，故天下莫能与之争。古之所谓曲则全者，岂虚言哉！诚全而归之。
希言自然，故飘风不终朝，骤雨不终日。孰为此者？天地。天地尚不能久，而况于人乎？故从事于道者，道者，同于道；德者，同于德；失者，同于失。同于道者，道亦乐得之；同于德者，德亦乐得之；同于失者，失亦乐得之。信不足，焉有不信焉。
企者不立；跨者不行；自见者不明；自是者不彰；自伐者无功；自矜者不长。其在道也，曰：馀食赘行。物或恶之，故有道者不处。
有物混成，先天地生。寂兮寥兮，独立不改，周行而不殆，可以为天下母。吾不知其名，字之曰道，强为之名曰大。大曰逝，逝曰远，远曰反。故道大，天大，地大，王亦大。域中有四大，而王居其一焉。人法地，地法天，天法道，道法自然。
重为轻根，静为躁君。是以圣人终日行不离辎重。虽有荣观，燕处超然。奈何万乘之主，而以身轻天下？轻则失本，躁则失君。
善行无辙迹，善言无瑕讁；善数不用筹策；善闭无关楗而不可开，善结无绳约而不可解。是以圣人常善救人，故无弃人；常善救物，故无弃物。是谓袭明。故善人者，不善人之师；不善人者，善人之资。不贵其师，不爱其资，虽智大迷，是谓要妙。
知其雄，守其雌，为天下溪。为天下溪，常德不离，复归于婴儿。知其白，守其黑，为天下式。为天下式，常德不忒，复归于无极。知其荣，守其辱，为天下谷。为天下谷，常德乃足，复归于朴。朴散则为器，圣人用之，则为官长，故大制不割。
将欲取天下而为之，吾见其不得已。天下神器，不可为也，为者败之，执者失之。故物或行或随；或歔或吹；或强或羸；或挫或隳。是以圣人去甚，去奢，去泰。
以道佐人主者，不以兵强天下。其事好还。师之所处，荆棘生焉。大军之后，必有凶年。善有果而已，不敢以取强。果而勿矜，果而勿伐，果而勿骄。果而不得已，果而勿强。物壮则老，是谓不道，不道早已。
夫佳兵者，不祥之器，物或恶之，故有道者不处。君子居则贵左，用兵则贵右。兵者不祥之器，非君子之器，不得已而用之，恬淡为上。胜而不美，而美之者，是乐杀人。夫乐杀人者，则不可以得志于天下矣。吉事尚左，凶事尚右。偏将军居左，上将军居右，言以丧礼处之。杀人之衆，以哀悲泣之，战胜以丧礼处之。
道常无名。天下莫能臣也。侯王若能守之，万物将自宾。天地相合，以降甘露，民莫之令而自均。始制有名，名亦既有，夫亦将知止，知止所以不殆。譬道之在天下，犹川谷之与江海。
知人者智，自知者明。胜人者有力，自胜者强。知足者富。强行者有志。不失其所者久。死而不亡者寿。
大道泛兮，其可左右。万物恃之而生而不辞，功成不名有。衣养万物而不为主，常无欲，可名于小；万物归焉，而不为主，可名为大。以其终不自为大，故能成其大。
执大象，天下往。往而不害，安平大。乐与饵，过客止。道之出口，淡乎其无味，视之不足见，听之不足闻，用之不足既。
将欲歙之，必固张之；将欲弱之，必固强之；将欲废之，必固兴之；将欲夺之，必固与之。是谓微明。柔弱胜刚强。鱼不可脱于渊，国之利器不可以示人。
道常无为而无不为。侯王若能守之，万物将自化。化而欲作，吾将镇之以无名之朴。无名之朴，夫亦将无欲。不欲以静，天下将自定。
上德不德，是以有德；下德不失德，是以无德。上德无为而无以为；下德为之而有以为。上仁为之而无以为；上义为之而有以为。上礼为之而莫之应，则攘臂而扔之。故失道而后德，失德而后仁，失仁而后义，失义而后礼。夫礼者，忠信之薄，而乱之首。前识者，道之华，而愚之始。是以大丈夫处其厚，不居其薄；处其实，不居其华。故去彼取此。
昔之得一者：天得一以清；地得一以宁；神得一以灵；谷得一以盈；万物得一以生；侯王得一以为天下贞。其致之，天无以清，将恐裂；地无以宁，将恐发；神无以灵，将恐歇；谷无以盈，将恐竭；万物无以生，将恐灭；侯王无以贵高将恐蹶。故贵以贱为本，高以下为基。是以侯王自称孤、寡、不谷。此非以贱为本耶？非乎？故致数誉无誉。不欲琭琭如玉，珞珞如石。
反者道之动；弱者道之用。天下万物生于有，有生于无。
上士闻道，勤而行之；中士闻道，若存若亡；下士闻道，大笑之。不笑不足以为道。故建言有之：明道若昧；进道若退；夷道若纇；上德若谷；太白若辱；广德若不足；建德若偷；质真若渝；大方无隅；大器晚成；大音希声；大象无形；道隐无名。夫唯道，善贷且成。
道生一，一生二，二生三，三生万物。万物负阴而抱阳，冲气以为和。人之所恶，唯孤、寡、不谷，而王公以为称。故物或损之而益，或益之而损。人之所教，我亦教之。强梁者不得其死，吾将以为教父。
天下之至柔，驰骋天下之至坚。无有入无间，吾是以知无为之有益。不言之教，无为之益，天下希及之。
名与身孰亲？身与货孰多？得与亡孰病？是故甚爱必大费；多藏必厚亡。知足不辱，知止不殆，可以长久。
大成若缺，其用不弊。大盈若冲，其用不穷。大直若屈，大巧若拙，大辩若讷。躁胜寒静胜热。清静为天下正。
天下有道，却走马以粪。天下无道，戎马生于郊。祸莫大于不知足；咎莫大于欲得。故知足之足，常足矣。
不出户知天下；不闚牖见天道。其出弥远，其知弥少。是以圣人不行而知，不见而名，不为而成。
为学日益，为道日损。损之又损，以至于无为。无为而无不为。取天下常以无事，及其有事，不足以取天下。
圣人无常心，以百姓心为心。善者，吾善之；不善者，吾亦善之；德善。信者，吾信之；不信者，吾亦信之；德信。圣人在天下，歙歙为天下浑其心，百姓皆注其耳目，圣人皆孩之。
出生入死。生之徒，十有三；死之徒，十有三；人之生，动之死地，十有三。夫何故？以其生，生之厚。盖闻善摄生者，陆行不遇兕虎，入军不被甲兵；兕无所投其角，虎无所措其爪，兵无所容其刃。夫何故？以其无死地。
道生之，德畜之，物形之，势成之。是以万物莫不尊道而贵德。道之尊，德之贵，夫莫之命常自然。故道生之，德畜之；长之育之；亭之毒之；养之覆之。生而不有，为而不恃，长而不宰，是谓玄德。
天下有始，以为天下母。既知其母，复知其子，既知其子，复守其母，没其不殆。塞其兑，闭其门，终身不勤。开其兑，济其事，终身不救。见小曰明，守柔曰强。用其光，复归其明，无遗身殃；是为习常。
使我介然有知，行于大道，唯施是畏。大道甚夷，而民好径。朝甚除，田甚芜，仓甚虚；服文彩，带利剑，厌饮食，财货有馀；是谓盗夸。非道也哉！
善建不拔，善抱者不脱，子孙以祭祀不辍。修之于身，其德乃真；修之于家，其德乃馀；修之于乡，其德乃长；修之于国，其德乃丰；修之于天下，其德乃普。故以身观身，以家观家，以乡观乡，以国观国，以天下观天下。吾何以知天下然哉？以此。
含德之厚，比于赤子。蜂虿虺蛇不螫，猛兽不据，攫鸟不搏。骨弱筋柔而握固。未知牝牡之合而全作，精之至也。终日号而不嗄，和之至也。知和曰常，知常曰明，益生曰祥。心使气曰强。物壮则老，谓之不道，不道早已。
知者不言，言者不知。塞其兑，闭其门，挫其锐，解其分，和其光，同其尘，是谓玄同。故不可得而亲，不可得而踈；不可得而利，不可得而害；不可得而贵，不可得而贱。故为天下贵。
以正治国，以奇用兵，以无事取天下。吾何以知其然哉？以此：天下多忌讳，而民弥贫；民多利器，国家滋昏；人多伎巧，奇物滋起；法令滋彰，盗贼多有。故圣人云：我无为，而民自化；我好静，而民自正；我无事，而民自富；我无欲，而民自朴。
其政闷闷，其民淳淳；其政察察，其民缺缺。祸兮福之所倚，福兮祸之所伏。孰知其极？其无正。正复为奇，善复为妖。人之迷，其日固久。是以圣人方而不割，廉而不刿，直而不肆，光而不耀。
治人事天莫若啬。夫唯啬，是谓早服；早服谓之重积德；重积德则无不克；无不克则莫知其极；莫知其极，可以有国；有国之母，可以长久；是谓深根固柢，长生久视之道。
治大国若烹小鲜。以道莅天下，其鬼不神；非其鬼不神，其神不伤人；非其神不伤人，圣人亦不伤人。夫两不相伤，故德交归焉。
大国者下流，天下之交，天下之牝。牝常以静胜牡，以静为下。故大国以下小国，则取小国；小国以下大国，则取大国。故或下以取，或下而取。大国不过欲兼畜人，小国不过欲入事人。夫两者各得其所欲，大者宜为下。
道者万物之奥。善人之宝，不善人之所保。美言可以市，尊行可以加人。人之不善，何弃之有？故立天子，置三公，虽有拱璧以先驷马，不如坐进此道。古之所以贵此道者何？不曰：以求得，有罪以免耶？故为天下贵。
为无为，事无事，味无味。大小多少，报怨以德。图难于其易，为大于其细；天下难事，必作于易，天下大事，必作于细。是以圣人终不为大，故能成其大。夫轻诺必寡信，多易必多难。是以圣人犹难之，故终无难矣。
其安易持，其未兆易谋。其脆易泮，其微易散。为之于未有，治之于未乱。合抱之木，生于毫末；九层之台，起于累土；千里之行，始于足下。为者败之，执者失之。是以圣人无为故无败；无执故无失。民之从事，常于几成而败之。慎终如始，则无败事，是以圣人欲不欲，不贵难得之货；学不学，复衆人之所过，以辅万物之自然，而不敢为。
古之善为道者，非以明民，将以愚之。民之难治，以其智多。故以智治国，国之贼；不以智治国，国之福。知此两者亦𥡴式。常知𥡴式，是谓玄德。玄德深矣，远矣，与物反矣，然后乃至大顺。
江海所以能为百谷王者，以其善下之，故能为百谷王。是以圣人欲上民，必以言下之；欲先民，必以身后之。是以圣人处上而民不重，处前而民不害。是以天下乐推而不厌。以其不争，故天下莫能与之争。
天下皆谓我道大，似不肖。夫唯大，故似不肖。若肖久矣。其细也夫！我有三宝，持而保之。一曰慈，二曰俭，三曰不敢为天下先。慈故能勇；俭故能广；不敢为天下先，故能成器长。今舍慈且勇；舍俭且广；舍后且先；死矣！夫慈以战则胜，以守则固。天将救之，以慈卫之。
善为士者，不武；善战者，不怒；善胜敌者，不与；善用人者，为之下。是谓不争之德，是谓用人之力，是谓配天古之极。
用兵有言：吾不敢为主，而为客；不敢进寸，而退尺。是谓行无行；攘无臂；扔无敌；执无兵。祸莫大于轻敌，轻敌几丧吾宝。故抗兵相加，哀者胜矣。
吾言甚易知，甚易行。天下莫能知，莫能行。言有宗，事有君。夫唯无知，是以不我知。知我者希，则我者贵。是以圣人被褐怀玉。
知不知上；不知知病。夫唯病病，是以不病。圣人不病，以其病病，是以不病。
民不畏威，则大威至。无狎其所居，无厌其所生。夫唯不厌，是以不厌。是以圣人自知不自见；自爱不自贵。故去彼取此。
勇于敢则杀，勇于不敢则活。此两者，或利或害。天之所恶，孰知其故？是以圣人犹难之。天之道，不争而善胜，不言而善应，不召而自来，繟然而善谋。天网恢恢，踈而不失。
民不畏死，奈何以死惧之？若使民常畏死，而为奇者，吾得执而杀之，孰敢？常有司杀者杀。夫司杀者，是大匠斫；夫代大匠斫者，希有不伤其手矣。
民之饥，以其上食税之多，是以饥。民之难治，以其上之有为，是以难治。民之轻死，以其求生之厚，是以轻死。夫唯无以生为者，是贤于贵生。
人之生也柔弱，其死也坚强。万物草木之生也柔脆，其死也枯槁。故坚强者死之徒，柔弱者生之徒。是以兵强则不胜，木强则共。强大处下，柔弱处上。
天之道，其犹张弓与？高者抑之，下者举之；有馀者损之，不足者补之。天之道，损有馀而补不足。人之道，则不然，损不足以奉有馀。孰能有馀以奉天下，唯有道者。是以圣人为而不恃，功成而不处，其不欲见贤。
天下莫柔弱于水，而攻坚强者莫之能胜，其无以易之。弱之胜强，柔之胜刚，天下莫不知，莫能行。是以圣人云：受国之垢，是谓社稷主；受国不祥，是谓天下王。正言若反。
和大怨，必有馀怨；安可以为善？是以圣人执左契，而不责于人。有德司契，无德司彻。天道无亲，常与善人。
小国寡民。使有什伯之器而不用；使民重死而不远徙。虽有舟舆，无所乘之，虽有甲兵，无所陈之。使民复结绳而用之，甘其食，美其服，安其居，乐其俗。邻国相望，鸡犬之声相闻，民至老死，不相往来。
信言不美，美言不信。善者不辩，辩者不善。知者不博，博者不知。圣人不积，既以为人己愈有，既以与人己愈多。天之道，利而不害；圣人之道，为而不争。";

pub fn marysue_benches(c: &mut Criterion) {
    let text = black_box(TEST_21);
    c.bench_function("MarySue: Encoding 20", move |x| x.iter(|| encode(text)));
    let text = black_box(TEST_195);
    c.bench_function("MarySue: Encoding 200", move |x| x.iter(|| encode(text)));
    let text = black_box(TEST_2909);
    c.bench_function("MarySue: Encoding 3000", move |x| x.iter(|| encode(text)));
    let text = black_box(TEST_19414);
    c.bench_function("MarySue: Encoding 20000", move |x| x.iter(|| encode(text)));
}
