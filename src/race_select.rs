use crate::character::*;
use crate::screen::*;
use eframe::egui;
use eframe::egui::{CtxRef};
use eframe::epi::Frame;


pub struct SelectRaceDesc {}

struct RaceCost {
    pub race : ExactRace,
    pub cost : i32
}

pub struct RaceSelectScreen {
    character : Character,
    costs : Vec<RaceCost>
}

impl RaceCost {
    pub fn new() -> Self {
        Self {
            race : ExactRace::Undefined,
            cost : 0_i32
        }
    }

    pub fn make_cost(
        race : &ExactRace,
        cost : &i32
    ) -> Self {
        Self {
            race : race.clone(),
            cost : cost.clone()
        }
    }
}

impl RaceSelectScreen {
    pub fn new(character : &Character) -> Self {

        let races = SelectRaceDesc::get_all_race();
        let important = SelectRaceDesc::get_important_abilities(&character.class);

        let mut costs = Vec::<RaceCost>::new();
        for race in races {
            let mut cost = 0_i32;
            let race_base = Character::get_ability_buf(&race);
            for imp in &important {
                match imp {
                    AbilityType::Charisma => {
                        cost += race_base.charisma.value;
                    }
                    AbilityType::Wisdom => {
                        cost += race_base.wisdom.value;
                    }
                    AbilityType::Intelligence => {
                        cost += race_base.intelligence.value;
                    }
                    AbilityType::Strength => {
                        cost += race_base.strength.value;
                    }
                    AbilityType::Dexterity => {
                        cost += race_base.dexterity.value;
                    }
                    AbilityType::Constitution => {
                        cost += race_base.constitution.value;
                    }
                }
            }

            costs.push(RaceCost::make_cost(&race, &cost));
        }

        costs.sort_by(|a, b| {
            if a.cost > b.cost {
                std::cmp::Ordering::Less
            } else if a.cost == b.cost {
                std::cmp::Ordering::Equal
            } else {
                std::cmp::Ordering::Greater
            }
        });

        Self {
            character : character.clone(),
            costs
        }
    }
}

impl Screen for RaceSelectScreen {
    
    fn update(&mut self, ctx: &CtxRef, frame: &mut Frame<'_>) -> ScreenResult {

        let mut res = ScreenResult::Ok;

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading("Пришло время решить судьбу твоего появления в мире. Выбери частью какого народа ты будешь являться:");
                    ui.separator();
                    ui.separator();
                    ui.separator();

                    for race_cost in &self.costs {
                        if race_cost.cost > 0 {
                            ui.heading(Character::get_race_name(&race_cost.race));
                            ui.separator();
                            ui.label(SelectRaceDesc::get_race_desc(&race_cost.race));
                            ui.label(format!("Соотвествие классу {:?} составляет [{}] (больше - лучше)", self.character.get_class_name(), race_cost.cost));
                            if ui.button("Выбрать").clicked() {
                                self.character.exact_race = race_cost.race.clone();
                                res = ScreenResult::NextScreen(
                                    Box::new(crate::abilities_set_screen::AutoSetAbilitiesScreen::new(&self.character))
                                );
                            }
                            ui.separator();
                        }
                    }
                });
            });
        });

        res
    }
}

impl SelectRaceDesc {

    pub fn get_all_race() -> Vec<ExactRace> {
        vec![
            ExactRace::DarkElf,
            ExactRace::Dragonborn,
            ExactRace::ForestElf,
            ExactRace::ForestGnome,
            ExactRace::HalfElf,
            ExactRace::HalfOrc,
            ExactRace::HighElf,
            ExactRace::HillDwarf,
            ExactRace::Human,
            ExactRace::LightfootHalfling,
            ExactRace::MountainDwarf,
            ExactRace::RockGnome,
            ExactRace::StoutHalfling,
            ExactRace::Tiefling,
        ]
    }

    pub fn get_race_desc(race : &ExactRace) -> String {
        match race {
            ExactRace::MountainDwarf => {
                String::from("Полные древнего величия королевства и вырезанные в толще гор чертоги, удары кирок и молотков, раздающиеся в глубоких шахтах и пылающий кузнечный горн, верность клану и традициям и пылающая ненависть к гоблинам и оркам — вот вещи, объединяющие всех дварфов. Смелые и выносливые дварфы известны как опытные воины, шахтёры, камнетёсы и металлурги. Хотя они и не превышают 5 футов (152 сантиметра) в высоту, дварфы настолько широкоплечие и плотные, что весят столько же, сколько превосходящий их в росте на 2 фута (60 сантиметров) человек. Их отвага и выносливость также не уступает представителям более высоких народов. Кожа у дварфов бывает от тёмно-коричневой до светлой, с красным оттенком. Наиболее распространённые оттенки — светло-коричневый или смуглый, как разные виды земли. Их волосы, которые они носят длинными, но собранными в простые причёски, обычно чёрного, серого или коричневого цвета, но у дварфов с бледной кожей часто встречаются рыжие волосы. Дварфы мужчины очень ценят свои бороды и тщательно за ними ухаживают
                Будучи горным дварфом, вы являетесь сильным и выносливым, приспособленным к жизни в суровой местности. Вы довольно высоки (по дварфской мерке), и скорее светлокожи. Щитовые дварфы из северного Фаэруна, а также правящий клан хиларов и благородный клан деваров из Саги о Копье, всё это горные дварфы")
            }
            ExactRace::HillDwarf => {
                String::from("Полные древнего величия королевства и вырезанные в толще гор чертоги, удары кирок и молотков, раздающиеся в глубоких шахтах и пылающий кузнечный горн, верность клану и традициям и пылающая ненависть к гоблинам и оркам — вот вещи, объединяющие всех дварфов. Смелые и выносливые дварфы известны как опытные воины, шахтёры, камнетёсы и металлурги. Хотя они и не превышают 5 футов (152 сантиметра) в высоту, дварфы настолько широкоплечие и плотные, что весят столько же, сколько превосходящий их в росте на 2 фута (60 сантиметров) человек. Их отвага и выносливость также не уступает представителям более высоких народов. Кожа у дварфов бывает от тёмно-коричневой до светлой, с красным оттенком. Наиболее распространённые оттенки — светло-коричневый или смуглый, как разные виды земли. Их волосы, которые они носят длинными, но собранными в простые причёски, обычно чёрного, серого или коричневого цвета, но у дварфов с бледной кожей часто встречаются рыжие волосы. Дварфы мужчины очень ценят свои бороды и тщательно за ними ухаживают
                Будучи холмовым дварфом вы обладаете обострёнными чувствами, развитой интуицией и замечательной стойкостью. Золотые дварфы Фаэруна, в их могучем южном королевстве являются холмовыми дварфами, также как и изгнанные нейдары и свихнувшиеся клары из Кринна (мир Саги о Копье).")
            }
            ExactRace::HighElf => {
                String::from("Эльфы это волшебный народ, обладающий неземным изяществом, живущий в мире, но не являющийся его частью. Они живут в местах, наполненных воздушной красотой, в глубинах древних лесов или в серебряных жилищах, увенчанных сверкающими шпилями и переливающихся волшебным светом. Там лёгкие дуновения ветра разносят обрывки тихих мелодий и нежные ароматы. Эльфы любят природу и магию, музыку и поэзию, и всё прекрасное, что есть в мире. Обладая неземным изяществом и тонкими чертами, эльфы кажутся людям и представителям других рас чересчур красивыми. В среднем, они немного ниже людей, их рост колеблется от 5 до 6 футов (от 150 до 185 сантиметров). Они стройнее людей, и весят от 100 до 145 фунтов (от 45 до 65 килограмм). Мужчины и женщины почти одинакового роста, и мужчины весят лишь незначительно больше. Цвета кожи у эльфов включают в себя все человеческие оттенки, а также цвета с медным, бронзовым и голубовато-белым отливом. Волосы помимо человеческих цветов могут быть зелёными или синими, а глаза приобретать цвет жидкого золота или серебра. У эльфов не растут волосы на лице, и почти отсутствуют волосы на теле. Они предпочитают элегантную одежду ярких цветов и простые, но красивые украшения. Поскольку вы — высший эльф, у вас острый ум и вы знакомы, по крайней мере, с основами магии. Во многих мирах D&D существует два вида высших эльфов. Один вид (который включает серых эльфов и эльфов долин Серого Ястреба, сильванести Саги о Копье и солнечных эльфов Забытых Королевств) высокомерен и замкнут, считая себя выше не-эльфов и даже других эльфов. Другой вид (включающий высших эльфов Серого Ястреба, квалинести из Саги о Копье и лунных эльфов из Забытых Королевств) более распространён и дружелюбен, и часто встречается среди людей и других рас. У солнечных эльфов Фаэруна (также называемых золотыми эльфами или эльфами восхода) бронзовая кожа и волосы медного, чёрного или золотистого оттенка. У них золотые, серебристые или чёрные глаза. Лунные эльфы (также называемые серебряными или серыми эльфами) гораздо бледнее, с алебастровой кожей, имеющей иногда оттенок синего. У них часто серебристо-белые, чёрные или синие волосы, но и различные оттенки светлых, коричневых и рыжих тонов также не являются редкими. У них синие или зелёные глаза с золотыми вкраплениями. ")
            }
            ExactRace::ForestElf => {
                String::from("Эльфы это волшебный народ, обладающий неземным изяществом, живущий в мире, но не являющийся его частью. Они живут в местах, наполненных воздушной красотой, в глубинах древних лесов или в серебряных жилищах, увенчанных сверкающими шпилями и переливающихся волшебным светом. Там лёгкие дуновения ветра разносят обрывки тихих мелодий и нежные ароматы. Эльфы любят природу и магию, музыку и поэзию, и всё прекрасное, что есть в мире. Обладая неземным изяществом и тонкими чертами, эльфы кажутся людям и представителям других рас чересчур красивыми. В среднем, они немного ниже людей, их рост колеблется от 5 до 6 футов (от 150 до 185 сантиметров). Они стройнее людей, и весят от 100 до 145 фунтов (от 45 до 65 килограмм). Мужчины и женщины почти одинакового роста, и мужчины весят лишь незначительно больше. Цвета кожи у эльфов включают в себя все человеческие оттенки, а также цвета с медным, бронзовым и голубовато-белым отливом. Волосы помимо человеческих цветов могут быть зелёными или синими, а глаза приобретать цвет жидкого золота или серебра. У эльфов не растут волосы на лице, и почти отсутствуют волосы на теле. Они предпочитают элегантную одежду ярких цветов и простые, но красивые украшения. Поскольку вы — лесной эльф, у вас обострённые чувства и интуиция, и ваши стремительные ноги несут вас быстро и незаметно через ваши родные леса. Эта категория включает диких эльфов Серого Ястреба и кагонести из Саги о Копье, а также расы, называемые лесными эльфами Серого Ястреба и Забытых Королевств. В Фаэруне лесные эльфы (также называемые дикими или зелёными) являются затворниками, не доверяющими не-эльфам. Кожа лесных эльфов, как правило, имеет медный оттенок, иногда со следами зелёного. У них часто коричневые и чёрные волосы, но иногда они бывают светлого или бронзового оттенков. У них зелёные, карие или орехового цвета глаза. ")
            }
            ExactRace::DarkElf => {
                String::from("Эльфы это волшебный народ, обладающий неземным изяществом, живущий в мире, но не являющийся его частью. Они живут в местах, наполненных воздушной красотой, в глубинах древних лесов или в серебряных жилищах, увенчанных сверкающими шпилями и переливающихся волшебным светом. Там лёгкие дуновения ветра разносят обрывки тихих мелодий и нежные ароматы. Эльфы любят природу и магию, музыку и поэзию, и всё прекрасное, что есть в мире. Обладая неземным изяществом и тонкими чертами, эльфы кажутся людям и представителям других рас чересчур красивыми. В среднем, они немного ниже людей, их рост колеблется от 5 до 6 футов (от 150 до 185 сантиметров). Они стройнее людей, и весят от 100 до 145 фунтов (от 45 до 65 килограмм). Мужчины и женщины почти одинакового роста, и мужчины весят лишь незначительно больше. Цвета кожи у эльфов включают в себя все человеческие оттенки, а также цвета с медным, бронзовым и голубовато-белым отливом. Волосы помимо человеческих цветов могут быть зелёными или синими, а глаза приобретать цвет жидкого золота или серебра. У эльфов не растут волосы на лице, и почти отсутствуют волосы на теле. Они предпочитают элегантную одежду ярких цветов и простые, но красивые украшения. Произошедшие от более древней подрасы темнокожих эльфов, дроу были изгнаны с земной поверхности мира, и обречены поклоняться богине Лолс и следовать пути зла и упадка. Теперь они построили свою цивилизацию в глубинах Подземья, устроенную согласно Пути Лолс. Также называемые тёмными эльфами, дроу имеют чёрную кожу, которая напоминает полированный обсидиан и совершенно белые или очень светлые волосы. У них обычно бледные глаза (настолько бледные, что могут показаться белыми) с сиреневым, серебряным, розовым, красным или синим оттенком. Они, как правило, меньше и стройнее, чем большинство эльфов. Искатели приключений дроу редки, и их раса существует не во всех мирах. Спросите вашего Мастера, можете ли вы играть персонажем дроу")
            }
            ExactRace::LightfootHalfling => {
                String::from("Целью большинства полуросликов является домашний уют. Место, где можно поселиться в покое и тишине, подальше от мародёрствующих чудовищ и сражающихся армий. Огонь очага, сытная пища, добрая выпивка и добрая беседа. Хотя некоторые полурослики проживают свой век в удалённых сельских общинах, другие сбиваются в постоянно кочующие общины, влекомые открытыми дорогами, широкими горизонтами и возможностью открыть чудеса новых мест и новых людей. Но даже такие кочевники любят покой, вкусную еду, свой очаг и свой дом, даже если это повозка, трясущаяся по пыльной дороге или плот, плывущий по течению реки. МАЛЕНЬКИЕ И ПРАКТИЧНЫЕ Крошечные полурослики выживают в мире, полном более крупных существ, стараясь избегать внимания, а если это оказывается невозможным, то избегая враждебности. Имея рост около 3 футов (90 сантиметров), они кажутся относительно безвредными, и благодаря этому успешно существуют столетиями, оставаясь в тени империй, войн и политической борьбы. Они склонны к полноте, и весят от 40 до 45 фунтов (от 18 до 20 килограмм). Кожа у полуросликов встречается от смуглой до бледной, с румянцем. Волосы обычно коричневые или рыже-коричневые, вьющиеся. Глаза полуросликов карие или ореховые. Мужчины часто отпускают длинные бакенбарды, но бороды носят редко, а усы тем более. Они любят носить простую, удобную одежду, предпочитая яркие цвета. Практичность полуросликов распространяется не только на их одежду. Они довольствуются удовлетворением основных потребностей и простых радостей, уделяя совсем мало внимания роскоши. Даже богатейшие из них предпочитают хранить своё добро в закрытых сундуках и подвалах, а не выставлять его на всеобщее обозрение. Полурослики умеют находить простые решения своих проблем, и являются весьма решительными. Легконогие полурослики умеют отлично скрываться, в том числе используя других существ как укрытие. Они приветливы и хорошо ладят с другими. В мире Забытых Королевств легконогие являются самой распространённой ветвью полуросликов. Легконогие более других склонны к перемене мест, и часто селятся по соседству с другими народами, или ведут кочевую жизнь. В мире Серого Ястреба таких полуросликов называют мохноногими или великанчиками.")
            }
            ExactRace::StoutHalfling => {
                String::from("Целью большинства полуросликов является домашний уют. Место, где можно поселиться в покое и тишине, подальше от мародёрствующих чудовищ и сражающихся армий. Огонь очага, сытная пища, добрая выпивка и добрая беседа. Хотя некоторые полурослики проживают свой век в удалённых сельских общинах, другие сбиваются в постоянно кочующие общины, влекомые открытыми дорогами, широкими горизонтами и возможностью открыть чудеса новых мест и новых людей. Но даже такие кочевники любят покой, вкусную еду, свой очаг и свой дом, даже если это повозка, трясущаяся по пыльной дороге или плот, плывущий по течению реки. МАЛЕНЬКИЕ И ПРАКТИЧНЫЕ Крошечные полурослики выживают в мире, полном более крупных существ, стараясь избегать внимания, а если это оказывается невозможным, то избегая враждебности. Имея рост около 3 футов (90 сантиметров), они кажутся относительно безвредными, и благодаря этому успешно существуют столетиями, оставаясь в тени империй, войн и политической борьбы. Они склонны к полноте, и весят от 40 до 45 фунтов (от 18 до 20 килограмм). Кожа у полуросликов встречается от смуглой до бледной, с румянцем. Волосы обычно коричневые или рыже-коричневые, вьющиеся. Глаза полуросликов карие или ореховые. Мужчины часто отпускают длинные бакенбарды, но бороды носят редко, а усы тем более. Они любят носить простую, удобную одежду, предпочитая яркие цвета. Практичность полуросликов распространяется не только на их одежду. Они довольствуются удовлетворением основных потребностей и простых радостей, уделяя совсем мало внимания роскоши. Даже богатейшие из них предпочитают хранить своё добро в закрытых сундуках и подвалах, а не выставлять его на всеобщее обозрение. Полурослики умеют находить простые решения своих проблем, и являются весьма решительными. Коренастые полурослики выносливее других и обладают некоторой устойчивостью к ядам. Поговаривают, что в их жилах течёт толика дварфской крови. В мире Забытых Королевств таких полуросликов зовут сильными сердцем, и чаще всего они встречаются на юге.")
            }
            ExactRace::Human => {
                String::from("В большинстве миров люди — это самая молодая из распространённых рас. Они поздно вышли на мировую сцену и живут намного меньше, чем дварфы, эльфы и драконы. Возможно, именно краткость их жизней заставляет их стремиться достигнуть как можно большего в отведённый им срок. А быть может, они хотят что-то доказать старшим расам, и поэтому создают могучие империи, основанные на завоеваниях и торговле. Что бы ни двигало ими, люди всегда были инноваторами и пионерами во всех мирах.")
            }
            ExactRace::Dragonborn => {
                String::from("Рождённые драконами, о чём говорит их название, драконорождённые идут гордо подняв голову по миру, который встречает их со страхом и непониманием. Сформированные драконьими богами или самими драконами, драконорождённые первоначально вылупились из драконьих яиц как новая раса, сочетающая в себе лучшие качества драконов и гуманоидов. Некоторые драконорождённые являются верными слугами истинных драконов, другие образуют ряды солдат в великих войнах, а некоторые ищут свою судьбу, не найдя себе призвания. ГОРДЫЙ ДРАКОНИЙ РОД Драконорождённые выглядят как драконы, стоящие на задних лапах и принявшие гуманоидную форму, потеряв при этом крылья и хвост. Первые из них обладали чешуёй оттенков, присущих драконам разных видов, но поколения непрерывного скрещивания привели к появлению единого цвета. Их маленькие, аккуратные чешуйки обычно цвета меди или латуни, иногда с алым, золотым, медно-зелёным или рыжим оттенком. Они высоки и крепки, часто достигая шести с половиной футов (двух метров) в высоту и обладая весом свыше 300 фунтов (130 килограмм). Их крепкие, четырёхпалые конечности снабжены пальцами, не уступающими по прочности когтям. Кровь определённых видов драконов чрезвычайно сильна в представителях некоторых кланов. Эти драконорождённые часто гордятся тем, что цвет их чешуи близок к цвету их прародителей — ярко красный, зелёный, синий или белый, глянцево-чёрный или блестяще-золотой, серебряный, латунный, медный или бронзовый. ")
            }
            ExactRace::ForestGnome => {
                String::from("Нескончаемый гул трудолюбия слышен там, где селятся сплочённые общества гномов. Гул пронзают и звуки погромче: то тут, то там раздаётся скрежет шестерней, отголоски взрыва, возгласы удивления или триумфа, и, особенно часто — звонкий смех. Гномы восторгаются жизнью, каждый миг наслаждаясь новым изобретением, открытием, исследованием, созиданием или шалостью. ЗАДОРНЫЙ ВИД Гномы очень энергичны, и кажется, что каждый сантиметр их крошечного тела излучает энтузиазм и жизнелюбие. В среднем гномы чуть выше 3 футов (90 сантиметров), и весят от 40 до 45 фунтов (от 18 до 20 килограмм). Их смуглые или коричневые лица обычно украшены широкими улыбками (над которыми нависают их выдающиеся носы), и их светлые глаза светятся возбуждением. Их русые волосы обычно торчат в разные стороны, словно выражая неослабевающий интерес ко всему на свете. Индивидуальность гномов ярко выражается в их внешности. Гномы мужчины содержат свои бороды, в отличие от растрёпанных волос, аккуратно подстриженными, но часто расчёсывают их на несколько прядей, или придают забавную заострённую форму. Их одежда, обычно спокойных коричневых тонов, изящно украшена вышивкой, тиснением, или расшита драгоценными камнями. Лесные гномы обладают природными способностями к иллюзии, и унаследовали проворство и скрытность. В мирах D&D лесные гномы встречаются редко, и являются скрытным народом. Они собираются в спрятанные в глубинах лесов общины, и используют иллюзию и обман, чтобы укрыться от опасности или скрыть свой побег в случае обнаружения. Лесные гномы обычно дружелюбны с другими добрыми лесными народами, и считают эльфов и добрых фей своими главными союзниками. Эти гномы также дружат с мелкими лесными зверушками, которые предупреждают их об опасности.")
            }
            ExactRace::RockGnome => {
                String::from("Нескончаемый гул трудолюбия слышен там, где селятся сплочённые общества гномов. Гул пронзают и звуки погромче: то тут, то там раздаётся скрежет шестерней, отголоски взрыва, возгласы удивления или триумфа, и, особенно часто — звонкий смех. Гномы восторгаются жизнью, каждый миг наслаждаясь новым изобретением, открытием, исследованием, созиданием или шалостью. ЗАДОРНЫЙ ВИД Гномы очень энергичны, и кажется, что каждый сантиметр их крошечного тела излучает энтузиазм и жизнелюбие. В среднем гномы чуть выше 3 футов (90 сантиметров), и весят от 40 до 45 фунтов (от 18 до 20 килограмм). Их смуглые или коричневые лица обычно украшены широкими улыбками (над которыми нависают их выдающиеся носы), и их светлые глаза светятся возбуждением. Их русые волосы обычно торчат в разные стороны, словно выражая неослабевающий интерес ко всему на свете. Индивидуальность гномов ярко выражается в их внешности. Гномы мужчины содержат свои бороды, в отличие от растрёпанных волос, аккуратно подстриженными, но часто расчёсывают их на несколько прядей, или придают забавную заострённую форму. Их одежда, обычно спокойных коричневых тонов, изящно украшена вышивкой, тиснением, или расшита драгоценными камнями. Скальные гномы выделяются своей изобретательностью и стойкостью. Большинство гномов в мирах D&D являются скальными, включая гномов-ремесленников из мира Саги о Копье.")
            }
            ExactRace::HalfElf => {
                String::from("Бродящие по двум мирам, но в действительности, не принадлежащие ни одному из них. Полуэльфы сочетают в себе, как некоторые говорят, лучшие качества обеих рас: человеческие любознательность, изобретательность и амбиции, приправленные изысканными чувствами, любовью к природе и ощущением прекрасного, свойственными эльфам. Некоторые полуэльфы живут среди людей, отгороженные эмоциональными и физическими различиями, наблюдая за старением друзей и возлюбленных, лишь слегка тронутые временем. Другие живут с эльфами в неподвластных времени эльфийских королевствах. Они стремительно растут, и достигают зрелости, пока их сверстники ещё остаются детьми. Многие полуэльфы не способны ужиться ни в одном обществе, и выбирают жизнь одиноких странников или объединяются с другими изгнанниками и неудачниками, чтобы отправиться к приключениям. МЕЖ ДВУХ МИРОВ Для людей полуэльфы выглядят эльфами, а для эльфов они люди. Их рост примерно посередине между людьми и эльфами, и составляет от 5 до 6 футов (от 155 до 183 сантиметров). Полуэльфы не такие стройные, как эльфы, но и не такие широкоплечие как люди. Их вес колеблется между 100 и 180 фунтами (45 и 80 килограммами), и мужчины лишь немного превосходят женщин в росте и весе. У мужчин полуэльфов на лицах растут волосы, и некоторые отращивают бороды, чтобы скрыть своё эльфийское происхождение. Цвет волос и кожи у них примерно посередине между их людскими и эльфийскими родителями, и таким образом разнообразие даже выше, чем у исходных рас. Цвет глаз они обычно наследуют у эльфийских родителей.")
            }
            ExactRace::HalfOrc => {
                String::from("Находясь ли под предводительством могучего колдуна, или стараясь установить мир после многолетнего конфликта, орки и племена людей иногда заключали союзы, объединяя силы в огромные орды, терроризирующие более цивилизованные государства по соседству. Когда такие союзы скреплялись узами брака, появлялись полуорки. Некоторые полуорки возвышались, становясь гордыми вождями племён. Их человечья кровь давала им преимущество над их чистокровными соперниками. Другие отправлялись в мир, чтоб доказать своё превосходство над представителями более цивилизованных народов. Многие из них становились искателями приключений, достигая величия благодаря своим могучим свершениям, и дурной славы, благодаря варварским нравам и дикарской ярости. КРЕПКИЕ И ПОКРЫТЫЕ ШРАМАМИ Серый цвет кожи, плоский лоб, выступающая челюсть, торчащие клыки и могучее телосложение не оставляет сомнений в орочьем происхождении для любого, кто смотрит на полуорка. Рост полуорков между 6 и 7 футами (183 и 213 сантиметрами), а вес обычно колеблется между 180 и 250 фунтов (80 и 113 килограммами). Орки гордятся своими боевыми шрамами, а узоры, составленные из шрамов, украшают их тела. Другие шрамы могут отмечать бывшего раба или изгнанника. Все полуорки, живущие с орками, или поблизости от них, имеют шрамы, следы унижений и поводы для гордости, отмечающие их победы и ранения. Живущие же среди людей полуорки могут как выставлять свои шрамы на обозрение с гордостью, так и со стыдом прятать их.")
            }
            ExactRace::Tiefling => {
                String::from("Быть тифлингом — значит постоянно натыкаться на пристальные взгляды и перешёптывания, терпеть страдания и оскорбления, видеть страх и недоверие в глазах каждого встречного. Беда в том, что тифлинги знают причину этого — договор, заключённый много поколений назад и позволивший Асмодею — владыке Девяти Преисподних — внести вклад в их родословную. Такая внешность и природа — не их вина, а последствие древнего прегрешения, расплачиваться за которое предстоит им, их детям, и детям их детей. ДЬЯВОЛЬСКАЯ РОДОСЛОВНАЯ Тифлинги произошли от людей, и по большому счёту всё ещё выглядят как люди. Тем не менее, их дьявольское наследие оставило заметный отпечаток на внешности. У тифлингов есть большие рога, принимающие различные формы: у некоторых витые рога, как у барана, у других длинные и прямые, как у газели, у третьих спиральные, как у антилопы. Их толстый хвост, от 120 до 150 сантиметров длиной, вьётся между ног, похлёстывая их, когда тифлинг нервничает. Острые зубы превращают улыбку в звериный оскал, а их глаза сплошного цвета без намёка на зрачок или белок, обычно чёрного, красного, белого, серебряного или золотого цвета. Цвет кожи тифлингов может принимать все тона, характерные для человека, а также различные оттенки красного. Их волосы, струящиеся из-за рогов, обычно тёмные, от чёрного или коричневого до тёмно-красного, синего или фиолетового. ")
            }
            ExactRace::Undefined => {
                String::from("Неизвестная раса")
            }
        }
    }

    pub fn get_important_abilities(class : &CharacterClass) -> Vec<AbilityType> {
        let mut res = Vec::<AbilityType>::new();

        match class {
            CharacterClass::Barbarian => {
                res.push(AbilityType::Strength);
                res.push(AbilityType::Constitution);
            }
            CharacterClass::Bard => {
                res.push(AbilityType::Charisma);
            }
            CharacterClass::Cleric => {
                res.push(AbilityType::Wisdom);
            }
            CharacterClass::Druid => {
                res.push(AbilityType::Wisdom);
            }
            CharacterClass::Fighter => {
                res.push(AbilityType::Strength);
                res.push(AbilityType::Dexterity);
                res.push(AbilityType::Constitution);
            }
            CharacterClass::Monk => {
                res.push(AbilityType::Dexterity);
                res.push(AbilityType::Wisdom);
            }
            CharacterClass::Paladin => {
                res.push(AbilityType::Strength);
                res.push(AbilityType::Charisma);
                res.push(AbilityType::Constitution);
            }
            CharacterClass::Ranger => {
                res.push(AbilityType::Dexterity);
                res.push(AbilityType::Wisdom);
            }
            CharacterClass::Rogue => {
                res.push(AbilityType::Dexterity);
            }
            CharacterClass::Sorcerer => {
                res.push(AbilityType::Charisma);
            }
            CharacterClass::Warlok => {
                res.push(AbilityType::Charisma);
            }
            CharacterClass::Wizard => {
                res.push(AbilityType::Intelligence);
            }
            CharacterClass::Undefined => {
            }
        }

        res
    }

    pub fn get_ability_order(class : &CharacterClass) -> Vec<AbilityType> {
        let mut res = Vec::<AbilityType>::new();

        match class {
            CharacterClass::Barbarian => {
                res.push(AbilityType::Strength);
                res.push(AbilityType::Constitution);
                res.push(AbilityType::Dexterity);
                res.push(AbilityType::Charisma);
                res.push(AbilityType::Wisdom);
                res.push(AbilityType::Intelligence);
            }
            CharacterClass::Bard => {
                res.push(AbilityType::Charisma);
                res.push(AbilityType::Dexterity);
                res.push(AbilityType::Constitution);
                res.push(AbilityType::Strength);
                res.push(AbilityType::Wisdom);
                res.push(AbilityType::Intelligence);
            }
            CharacterClass::Cleric => {
                res.push(AbilityType::Wisdom);
                res.push(AbilityType::Constitution);
                res.push(AbilityType::Charisma);
                res.push(AbilityType::Dexterity);
                res.push(AbilityType::Strength);
                res.push(AbilityType::Intelligence);
            }
            CharacterClass::Druid => {
                res.push(AbilityType::Wisdom);
                res.push(AbilityType::Constitution);
                res.push(AbilityType::Charisma);
                res.push(AbilityType::Dexterity);
                res.push(AbilityType::Strength);
                res.push(AbilityType::Intelligence);
            }
            CharacterClass::Fighter => {
                res.push(AbilityType::Strength);
                res.push(AbilityType::Constitution);
                res.push(AbilityType::Dexterity);
                res.push(AbilityType::Wisdom);
                res.push(AbilityType::Charisma);
                res.push(AbilityType::Intelligence);
            }
            CharacterClass::Monk => {
                res.push(AbilityType::Dexterity);
                res.push(AbilityType::Wisdom);
                res.push(AbilityType::Constitution);
                res.push(AbilityType::Wisdom);
                res.push(AbilityType::Charisma);
                res.push(AbilityType::Intelligence);
            }
            CharacterClass::Paladin => {
                res.push(AbilityType::Strength);
                res.push(AbilityType::Charisma);
                res.push(AbilityType::Constitution);
                res.push(AbilityType::Dexterity);
                res.push(AbilityType::Wisdom);
                res.push(AbilityType::Intelligence);
            }
            CharacterClass::Ranger => {
                res.push(AbilityType::Dexterity);
                res.push(AbilityType::Wisdom);
                res.push(AbilityType::Constitution);
                res.push(AbilityType::Intelligence);
                res.push(AbilityType::Charisma);
                res.push(AbilityType::Strength);
            }
            CharacterClass::Rogue => { 
                res.push(AbilityType::Dexterity);
                res.push(AbilityType::Wisdom);
                res.push(AbilityType::Constitution);
                res.push(AbilityType::Intelligence);
                res.push(AbilityType::Charisma);
                res.push(AbilityType::Strength);
            }
            CharacterClass::Sorcerer => {
                res.push(AbilityType::Charisma);
                res.push(AbilityType::Intelligence);
                res.push(AbilityType::Wisdom);
                res.push(AbilityType::Constitution);
                res.push(AbilityType::Dexterity);
                res.push(AbilityType::Strength);
            }
            CharacterClass::Warlok => {
                res.push(AbilityType::Charisma);
                res.push(AbilityType::Intelligence);
                res.push(AbilityType::Wisdom);
                res.push(AbilityType::Constitution);
                res.push(AbilityType::Dexterity);
                res.push(AbilityType::Strength);
            }
            CharacterClass::Wizard => {
                res.push(AbilityType::Intelligence);
                res.push(AbilityType::Wisdom);
                res.push(AbilityType::Constitution);
                res.push(AbilityType::Dexterity);
                res.push(AbilityType::Strength);
                res.push(AbilityType::Charisma);
            }
            CharacterClass::Undefined => {
            }
        }

        res
    }
}