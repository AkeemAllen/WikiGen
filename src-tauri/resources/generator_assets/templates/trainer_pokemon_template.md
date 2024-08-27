<div class="trainer-pokemon-card">
  <div style="display: grid; row-gap: 0.5rem">
    <img src="../../img/pokemon/{{pokemon_name}}.png" alt={{pokemon_name}} style="border-radius: 10px; background-color: #fff; justify-self: center;"/>
    <div style="display: flex; flex-direction: row; background-color: white; justify-content: space-between; border-radius: 10px; padding:2px; align-items: center;">
      <a href="/route-testing/pokemon/{{page_title}}">{{cap_pokemon_name}}</a>
      Lv {{level}}
    </div>
  </div>
  <div class="trainer-pokemon-attributes">
    <div>
      <img src="../../img/types/normal.png" alt=eevee style="width: 50px;"/>
      <img src="../../img/types/water.png" alt=eevee style="width: 50px;"/>
    </div>
    <div>
      <p style="margin: 0px; font-size: 10px;">Ability:</p>
      {{ability}}
    </div>
    <div>
      <p style="margin: 0px; font-size: 10px;">Nature:</p>
      {{nature}}
    </div>
    <div>
      <p style="margin: 0px; font-size: 10px;">Held Item:</p>
      <div style="display: flex; align-items: center">
        {{item_image}}
        {{item_name}}
      </div>
    </div>
  </div>
  <div class="trainer-pokemon-moveset">
    {{move_1}}
    {{move_2}}
    {{move_3}}
    {{move_4}}
  </div>
</div>
