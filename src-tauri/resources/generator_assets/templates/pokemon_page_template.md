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
      <td style="border-top: none; width: 70px">{{hp}}</td>
      <td style="width: 100%; min-width: 450px; border-top: none;">
        <div style="width: {{hp_width}}%;" class="ranking-bar rank-{{hp_rank}}">
        </div>
      </td>
    </tr>
    <tr style="display: flex; align-items: center;">
      <th style="color: #737373;">Attack</th>
      <td style="border-top: none; width: 70px">{{attack}}</td>
      <td style="width: 100%; min-width: 450px; border-top: none;">
        <div style="width: {{atk_width}}%;" class="ranking-bar rank-{{atk_rank}}">
        </div>
      </td>
    </tr>
    <tr style="display: flex; align-items: center;">
      <th style="color: #737373;">Defense</th>
      <td style="border-top: none; width: 70px">{{defense}}</td>
      <td style="width: 100%; min-width: 450px; border-top: none;">
        <div style="width: {{def_width}}%;" class="ranking-bar rank-{{def_rank}}">
        </div>
      </td>
    </tr>
    <tr style="display: flex; align-items: center;">
      <th style="color: #737373;">SP Attack</th>
      <td style="border-top: none; width: 70px">{{special_attack}}</td>
      <td style="width: 100%; min-width: 450px; border-top: none;">
        <div style="width: {{sp_atk_width}}%;" class="ranking-bar rank-{{sp_atk_rank}}">
        </div>
      </td>
    </tr>
    <tr style="display: flex; align-items: center;">
      <th style="color: #737373;">SP Defense</th>
      <td style="border-top: none; width: 70px">{{special_defense}}</td>
      <td style="width: 100%; min-width: 450px; border-top: none;">
        <div style="width: {{sp_def_width}}%;" class="ranking-bar rank-{{sp_def_rank}}">
        </div>
      </td>
    </tr>
    <tr style="display: flex; align-items: center;">
      <th style="color: #737373;">Speed</th>
      <td style="border-top: none; width: 70px">{{speed}}</td>
      <td style="width: 100%; min-width: 450px; border-top: none;">
        <div style="width: {{speed_width}}%;" class="ranking-bar rank-{{speed_rank}}">
        </div>
      </td>
    </tr>
  </tbody>
</table>

{{evolution_change}}

{{locations}}

## Moveset

=== "Level Up Moves"
    {{level_up_moves}}

=== "Machine Moves"
    {{machine_moves}}
