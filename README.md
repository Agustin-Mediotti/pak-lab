<!--badges-->
[![coverage](https://github.com/Agustin-Mediotti/pak-lab/actions/workflows/coverage.yml/badge.svg?branch=master)](https://github.com/Agustin-Mediotti/pak-lab/actions/workflows/coverage.yml)
[![tests](https://github.com/Agustin-Mediotti/pak-lab/actions/workflows/tests.yml/badge.svg)](https://github.com/Agustin-Mediotti/pak-lab/actions/workflows/tests.yml)
[![codecov](https://codecov.io/gh/Agustin-Mediotti/pak-lab/branch/master/graph/badge.svg?token=XFOSKOMR8T)](https://codecov.io/gh/Agustin-Mediotti/pak-lab)

<!--banner (i love banners)-->
<pre align="center">
                                    ..                         ..                       ..    
                              < .z@8"`                   x .d88"                  . uW8"      
 .d``                          !@88E                      5888R                   `t888       
 @8Ne.   .u           u        '888E   u                  '888R           u        8888   .   
 %8888:u@88N       us888u.      888E u@8NL                 888R        us888u.     9888.z88N  
  `888I  888.   .@88 "8888"     888E`"88*"                 888R     .@88 "8888"    9888  888E 
   888I  888I   9888  9888      888E .dN.                  888R     9888  9888     9888  888E 
   888I  888I   9888  9888      888E~8888                  888R     9888  9888     9888  888E 
 uW888L  888'   9888  9888      888E '888&    88888888     888R     9888  9888     9888  888E 
'*88888Nu88P    9888  9888      888E  9888.   88888888    .888B .   9888  9888    .8888  888" 
~ '88888F`      "888*""888"   '"888*" 4888"               ^*888%    "888*""888"    `%888*%"   
   888 ^         ^Y"   ^Y'       ""    ""                   "%       ^Y"   ^Y'        "`      
   *8E                                                                                        
   '8>                                                                                        
    "                                                                                         
</pre>
<!--Hosteado en: **📦 `https://pak.com.ar`**-->

Backend de mi servidor personal, desarrollado en Rust con el framework Actix-Web. Este servidor impulsa un blog y un boletín sobre **política nacional argentina** y **soberanía digital**, con el objetivo de fomentar una visión crítica sobre el desarrollo tecnológico y político del país.

Este proyecto sigue la arquitectura propuesta en el libro [**“Zero to Production in Rust”**](https://www.zero2prod.com/), haciendo énfasis en buenas prácticas como pruebas automatizadas, separación de capas, y configuración robusta.

## Caracteristicas

- API RESTful escrita en Rust
- Persistencia con PostgreSQL vía SQLx
- Enrutamiento y servidor web con Actix-Web
- Migraciones de base de datos integradas
- Tareas asíncronas con Tokio
- Configuración flexible con `config`
- Uso de UUIDs y timestamps con `uuid` y `chrono`

## Sobre el proyecto

**`pak-lab`** forma parte de una iniciativa autogestionada que busca promover contenidos de análisis político y técnico desde una perspectiva soberana. Todo corre en un servidor personal y reciblado, mantenido con software libre y una infraestructura abierta y documentada.

<!--la gloriosa estrella federal-->
<p align="center">
  <img width="143" height="144" alt="pixel_art_small" src="https://github.com/user-attachments/assets/e75378a5-67f4-491d-904f-faff0f9a3e26" />
</p>

--- 

<!--footer-->
Licencia: MIT o Apache-2.0  
Contacto: [pak.mailbox.adm@gmail.com](mailto:pak.mailbox.adm@gmail.com)
