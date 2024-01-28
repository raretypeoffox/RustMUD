
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
      GetNewClass,
      GetNewOrigins,
      ReadMotd,
    }

    #[derive(PartialEq)]
    pub enum Race {
      None,
      Cragkin,
      Moonshade,
      Etherial,
      Starfolk,
      Frostling,
      Auroran,
    }

    #[derive(PartialEq)]
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

