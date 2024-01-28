
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
      GetNewOrigins,
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

