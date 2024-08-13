<div style="display: flex; flex-direction: row; column-gap: 3rem; align-content: center;">
  <img src="../../img/pokemon/{{pokemon_name}}.png" width="100"/>

  <div>
    <div style="display: flex; flex-direction: row; column-gap: 3rem; alignt-items: center; margin-bottom: 0.5rem;">
      <p style="color: #737373; margin: 0px; font-size: 16px; font-weight: normal;">Types</p>
      <div style="display: flex; flex-direction: row; align-items: center; column-gap: 1rem">
        {{type_1_image}}
        {{type_2_image}}
      </div>
    </div>
    <div style="display: flex; flex-direction: row; column-gap: 3rem; alignt-items: center; ">
      <p style="color: #737373; margin: 0px;  font-weight: normal; font-size:16px;">Abilities</p>
      <div style="display: flex; flex-direction: row; align-items: center; font-size: 16px">
        {{ability_1}}
        {{ability_2}}
      </div>
    </div>
  </div>
</div>

## Base Stats
<table style="width: 100%">
  <tbody style="width: 100%;">
    <tr style="display: flex; align-items: center;">
      <th style="color: #737373;" >HP</th>
      <td style="border-top: none; width: 50px">{{hp}}</td>
      <td style="width: 100%; min-width: 450px; border-top: none;">
        <div style="width: {{hp_width}}%;" class="ranking-bar rank-{{hp_rank}}">
        </div>
      </td>
    </tr>
    <tr style="display: flex; align-items: center;">
      <th style="color: #737373;">Attack</th>
      <td style="border-top: none; width: 50px">{{attack}}</td>
      <td style="width: 100%; min-width: 450px; border-top: none;">
        <div style="width: {{atk_width}}%;" class="ranking-bar rank-{{atk_rank}}">
        </div>
      </td>
    </tr>
    <tr style="display: flex; align-items: center;">
      <th style="color: #737373;">Defense</th>
      <td style="border-top: none; width: 50px">{{defense}}</td>
      <td style="width: 100%; min-width: 450px; border-top: none;">
        <div style="width: {{def_width}}%;" class="ranking-bar rank-{{def_rank}}">
        </div>
      </td>
    </tr>
    <tr style="display: flex; align-items: center;">
      <th style="color: #737373;">SP Attack</th>
      <td style="border-top: none; width: 50px">{{special_attack}}</td>
      <td style="width: 100%; min-width: 450px; border-top: none;">
        <div style="width: {{sp_atk_width}}%;" class="ranking-bar rank-{{sp_atk_rank}}">
        </div>
      </td>
    </tr>
    <tr style="display: flex; align-items: center;">
      <th style="color: #737373;">SP Defense</th>
      <td style="border-top: none; width: 50px">{{special_defense}}</td>
      <td style="width: 100%; min-width: 450px; border-top: none;">
        <div style="width: {{sp_def_width}}%;" class="ranking-bar rank-{{sp_def_rank}}">
        </div>
      </td>
    </tr>
    <tr style="display: flex; align-items: center;">
      <th style="color: #737373;">Speed</th>
      <td style="border-top: none; width: 50px">{{speed}}</td>
      <td style="width: 100%; min-width: 450px; border-top: none;">
        <div style="width: {{speed_width}}%;" class="ranking-bar rank-{{speed_rank}}">
        </div>
      </td>
    </tr>
  </tbody>
</table>

## Moveset

=== "Level Up Moves"
    | Level | Name | Power | Accuracy | PP | Type | Damage Class |
    | -- | -- | -- | -- | -- | -- | -- |
    | 1 | Tackle | 40 | 100 | 35 | ![normal](../img/types/normal.png) | ![physical](../img/types/physical.png) |
    | 1 | Growl | - | 100 | 40 | ![normal](../img/types/normal.png) | ![status](../img/types/status.png) |
    | 3 | Fairy-lock | - | - | 10 | ![fairy](../img/types/fairy.png) | ![status](../img/types/status.png) |
    | 4 | Secret-power | 70 | 100 | 20 | ![normal](../img/types/normal.png) | ![physical](../img/types/physical.png) |
    | 5 | Vine-whip | 45 | 100 | 25 | ![grass](../img/types/grass.png) | ![physical](../img/types/physical.png) |
    | 9 | Leech-seed | - | 90 | 10 | ![grass](../img/types/grass.png) | ![status](../img/types/status.png) |
    | 14 | Sleep-powder | - | 75 | 15 | ![grass](../img/types/grass.png) | ![status](../img/types/status.png) |
    | 14 | Poison-powder | - | 75 | 35 | ![poison](../img/types/poison.png) | ![status](../img/types/status.png) |
    | 18 | Take-down | 90 | 85 | 20 | ![normal](../img/types/normal.png) | ![physical](../img/types/physical.png) |
    | 21 | Sweet-scent | - | 100 | 20 | ![normal](../img/types/normal.png) | ![status](../img/types/status.png) |
    | 23 | Razor-leaf | 55 | 95 | 25 | ![grass](../img/types/grass.png) | ![physical](../img/types/physical.png) |
    | 27 | Growth | - | - | 20 | ![normal](../img/types/normal.png) | ![status](../img/types/status.png) |
    | 31 | Worry-seed | - | 100 | 10 | ![grass](../img/types/grass.png) | ![status](../img/types/status.png) |
    | 32 | Double-edge | 120 | 100 | 15 | ![normal](../img/types/normal.png) | ![physical](../img/types/physical.png) |
    | 33 | Synthesis | - | - | 5 | ![grass](../img/types/grass.png) | ![status](../img/types/status.png) |
    | 37 | Seed-bomb | 80 | 100 | 15 | ![grass](../img/types/grass.png) | ![physical](../img/types/physical.png) |

=== "Machine Moves"
    | Machine | Name | Power | Accuracy | PP | Type | Damage Class |
    | -- | -- | -- | -- | -- | -- | -- |
    | TM27 | Toxic | - | 90 | 10 | ![poison](../img/types/poison.png) | ![status](../img/types/status.png) |
    | TM100 | Confide | - | - | 20 | ![normal](../img/types/normal.png) | ![status](../img/types/status.png) |
    | TM27 | Return | - | 100 | 20 | ![normal](../img/types/normal.png) | ![physical](../img/types/physical.png) |
    | TM87 | Swagger | - | 85 | 15 | ![normal](../img/types/normal.png) | ![status](../img/types/status.png) |
    | TM09 | Venoshock | 65 | 100 | 10 | ![poison](../img/types/poison.png) | ![special](../img/types/special.png) |
    | TM05 | Rest | - | - | 5 | ![psychic](../img/types/psychic.png) | ![status](../img/types/status.png) |
    | TM36 | Sludge-bomb | 90 | 100 | 10 | ![poison](../img/types/poison.png) | ![special](../img/types/special.png) |
    | TM01 | Headbutt | 70 | 100 | 15 | ![normal](../img/types/normal.png) | ![physical](../img/types/physical.png) |
    | TM88 | Sleep-talk | - | - | 10 | ![normal](../img/types/normal.png) | ![status](../img/types/status.png) |
    | TM32 | Double-team | - | - | 15 | ![normal](../img/types/normal.png) | ![status](../img/types/status.png) |
    | TM86 | Grass-knot | - | 100 | 20 | ![grass](../img/types/grass.png) | ![special](../img/types/special.png) |
    | TM10 | Hidden-power | 60 | 100 | 15 | ![normal](../img/types/normal.png) | ![special](../img/types/special.png) |
    | TM21 | Frustration | - | 100 | 20 | ![normal](../img/types/normal.png) | ![physical](../img/types/physical.png) |
    | TM53 | Energy-ball | 90 | 100 | 10 | ![grass](../img/types/grass.png) | ![special](../img/types/special.png) |
    | TM45 | Attract | - | 100 | 15 | ![normal](../img/types/normal.png) | ![status](../img/types/status.png) |
    | TM45 | Solar-beam | 120 | 100 | 10 | ![grass](../img/types/grass.png) | ![special](../img/types/special.png) |
    | TM11 | Sunny-day | - | - | 5 | ![fire](../img/types/fire.png) | ![status](../img/types/status.png) |
    | TM08 | Substitute | - | - | 10 | ![normal](../img/types/normal.png) | ![status](../img/types/status.png) |
    | TM16 | Light-screen | - | - | 30 | ![psychic](../img/types/psychic.png) | ![status](../img/types/status.png) |
    | TM39 | Outrage | 120 | 100 | 10 | ![dragon](../img/types/dragon.png) | ![physical](../img/types/physical.png) |
    | TM20 | Safeguard | - | - | 25 | ![normal](../img/types/normal.png) | ![status](../img/types/status.png) |
    | TM07 | Protect | - | - | 10 | ![normal](../img/types/normal.png) | ![status](../img/types/status.png) |
    | TM12 | Facade | 70 | 100 | 20 | ![normal](../img/types/normal.png) | ![physical](../img/types/physical.png) |
    | TM01 | Work-up | - | - | 30 | ![normal](../img/types/normal.png) | ![status](../img/types/status.png) |
    | TM96 | Nature-power | - | - | 20 | ![normal](../img/types/normal.png) | ![status](../img/types/status.png) |
    | TM48 | Round | 60 | 100 | 15 | ![normal](../img/types/normal.png) | ![special](../img/types/special.png) |
    | TM49 | Echoed-voice | 40 | 100 | 15 | ![normal](../img/types/normal.png) | ![special](../img/types/special.png) |
    | TM53 | Mega-drain | 40 | 100 | 15 | ![grass](../img/types/grass.png) | ![special](../img/types/special.png) |
    | TM33 | Reflect | - | - | 20 | ![psychic](../img/types/psychic.png) | ![status](../img/types/status.png) |
    | TM75 | Swords-dance | - | - | 20 | ![normal](../img/types/normal.png) | ![status](../img/types/status.png) |
