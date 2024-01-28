
#![allow(dead_code)]

pub mod constants {

pub const GREETING: &'static str = r#"
                              /\
                              ||
                              ||
                              ||
                              ||                                               ~-----~
                              ||                                            /===--  ---~~~
                              ||                   ;'                 /==~- --   -    ---~~~
                              ||                (/ ('              /=----         ~~_  --(  '
                              ||             ' / ;'             /=----               \__~
         '                  ~==_=~          '('             ~-~~      ~~~~        ~~~--\~'
         \\\\                  (c_\_        .i.             /~--    ~~~--   -~     (     '
            `\               (}| /       / : \           / ~~------~     ~~\   (
            \ '               ||/ \      |===|          /~/             ~~~ \ \(
            ``~\              ~~\  )~.~_ >._.< _~-~     |`_          ~~-~     )\\
             '-~                 {  /  ) \___/ (   \   |` ` _       ~~         '
             \ -~\                -<__/  -   -  L~ -;   \\\\    \ _ _/
             `` ~~=\                  {    :    }\ ,\    ||   _ :(
                \  ~~=\__                \ _/ \_ /  )  } _//   ( `|'
                ``    , ~\--~=\           \     /  / _/ / '    (   '
                 \`    } ~ ~~ -~=\   _~_  / \ / \ )^ ( // :_  / '
                 |    ,          _~-'   '~~__-_  / - |/     \ (
                    \  ,_--_     _/              \_'---', -~ .   \\
                     )/      /\ / /\   ,~,         \__ _}     \_  "~_
                     ,      { ( _ )'} ~ - \_    ~\  (-:-)       "\   ~ 
                            /'' ''  )~ \~_ ~\   )->  \ :|    _,       " 
                           (\  _/)''} | \~_ ~  /~(   | :)   /          }
                          <``  >;,,/  )= \~__ {{{ '  \ =(  ,   ,       ;
                         {o_o }_/     |v  '~__  _    )-v|  "  :       ,"
                         {/"\_)       {_/'  \~__ ~\_ \\\\_} '  {        /~\\
                         ,/!          '_/    '~__ _-~ \_' :  '      ,"  ~ 
                        (''`                  /,'~___~    | /     ,"  \ ~' 
                       '/, )                 (-)  '~____~";     ,"     , }
                     /,')                    / \         /  ,~-"       '~'
                 (  ''/                     / ( '       /  /          '~'
              ~ ~  ,, /) ,                 (/( \)      ( -)          /~'
            (  ~~ )`  ~}                   '  \)'     _/ /           ~'
           { |) /`,--.(  }'                    '     (  /          /~'
          (` ~ ( c|~~| `}   )                        '/:\         ,'
           ~ )/``) )) '|),                          (/ | \)                 -sjm
           (` (-~(( `~`'  )                        ' (/ '
             `~'    )'`')                              '
               ` ``

                                **** Welcome to Mystic Realms! ****
                               **** Based on RustMUD by Vagonuth ****
"#;

pub const RACE_MSG: &'static str = r#"
Please choose from one of the following races:

[Cragkin]     Stone-skinned humanoids from mountainous regions, known for their
              incredible strength and ability to merge with rock and earth.

[Moonshades]  Nocturnal beings with a strong affinity to moonlight. They have
              enhanced night vision, silent movement and a knack for archery.

[Etherials]   Ghost-like beings that exist partially in another dimension, making them
              intangible and difficult to harm. They excel at hiding in the shadows.
              
[Starfolk]    Mysterious beings that come from the night sky, 
              their bodies resembling the starry heavens, with abilities tied to the cosmos.

[Frostlings]  Icy beings from the coldest parts of the world,
              capable of withstanding extreme cold and manipulating ice and snow.

[Aurorans]    Beings of pure light, they are the embodiment of goodness and purity,
              and are capable of great feats of healing and protection.

"#;

pub const ORIGIN_MSG: &'static str = r#"
Please choose from one of the following origins:

1. [Warrior of the Forgotten Legion]  
    A descendant of an ancient and legendary army, known for their unmatched 
    battle prowess and endurance.
              
2. [Elemental Envoy]
    Born under a rare celestial alignment, naturally attuned to elemental 
    forces, giving them a natural proclivty towards elemental magic.

3. [Spiritual Wanderer]
    Travelled through sacred lands, gaining unique insights into the 
    spiritual world, enhancing their healing and protective spells.

4. [Shadow Guild Operative]
    Trained by an elusive and feared guild, adept in the arts of stealth, 
    thievery, and assassination. Live in the shadows.
    
5. [Borderland Sentinel]
    Guarded the frontiers of their homeland, adept in using a 
    bow for long-range defense and keenly aware of their surroundings.

6. [Wandering Bard]
    Travelled far and wide, collecting tales and skills from various 
    cultures, versatile in a range of practical and social abilities.

Enter 1-6 to choose an origin:
"#;

pub const MOTD_MSG: &'static str = r#"
Welcome to Mystic Realms!

Press <enter> to continue...
"#;



    // pub const CON_PLAYING: u8 = 0;
    // pub const CON_GET_NAME: u8 = 1;
    // pub const CON_GET_OLD_PASSWORD: u8 = 2;
    // pub const CON_CONFIRM_NEW_NAME: u8 = 3;
    // pub const CON_GET_NEW_PASSWORD: u8 = 4;
    // pub const CON_CONFIRM_NEW_PASSWORD: u8 = 5;
    // pub const CON_GET_NEW_SEX: u8 = 6;
    // pub const CON_GET_NEW_CLASS: u8 = 7;
    // pub const CON_GET_NEW_ORIGINS: u8 = 8;
    // pub const CON_READ_MOTD: u8 = 9;

    #[derive(PartialEq, Copy, Clone)]                          
    pub enum Conn {
      Playing,
      GetName,
      GetPassword,
      ConfirmNewName,
      GetNewPassword,
      ConfirmNewPassword,
      GetNewSex,
      GetNewRace,
      GetNewOrigin,
      ReadMotd,
    }

    #[derive(PartialEq, Copy, Clone)]      
    pub enum Sex {
      None,
      Male,
      Female,
      Neutral,
    }

    #[derive(PartialEq, Copy, Clone)]      
    pub enum Race {
      None,
      Cragkin,
      Moonshade,
      Etherial,
      Starfolk,
      Frostling,
      Auroran,
    }

    #[derive(PartialEq, Copy, Clone)]      
    pub enum Origin {
      None,
      WarriorOfTheForgottenLegion,
      ElementalEnvoy,
      SpiritualWanderer,
      ShadowGuildOperative,
      BorderlandSentinel,
      WanderingBard,
    }
}

