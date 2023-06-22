<script setup>
import { ref, onMounted } from "vue";
let isActive = ref(false);
let showNav = ref(true);
let lastScrollPosition = ref(0);

function toggleMenu (){
  isActive.value = !isActive.value;
  console.log('toggleMenu -[ isActive: ' + isActive.value + '] [showNav: ' + showNav.value + ']');

}
function toggleNav (){
  showNav.value = !showNav.value;
  console.log('toggleMenu -[ isActive: ' + isActive.value + '] [showNav: ' + showNav.value + ']');

}

</script>

<template>
  <nav class="navbar" role="navigation" aria-label="main navigation">
    <div class="navbar-brand">
        <NuxtLink class="navbar-item" to="/"> <h2 class="is-large">Ruud Erie </h2></NuxtLink>
    </div>
    <NuxtLink role="button" 
    :class="{ 'is-active': isActive }" 
    class="navbar-burger" 
    aria-label="menu" aria-expanded="false" 
    data-target="navbarStandard"
    @click="toggleMenu">
      <span aria-hidden="true"></span>
      <span aria-hidden="true"></span>
      <span aria-hidden="true"></span>
    </NuxtLink>
    <div id="navbarStandard" class="navbar-menu" :class="{'is-active': isActive}" v-show="{ isActive }">
      <div class="navbar-start"></div>
    <div :class="{ 'is-hidden': !showNav }">
      <div class="navbar-end">
        <NuxtLink to="/" class="navbar-item"> Home </NuxtLink>
        <NuxtLink to="/insights" class="navbar-item"> Insights </NuxtLink>
        <NuxtLink to="/about" class="navbar-item"> About </NuxtLink>
        <span class="navbar-item">
              <NuxtLink to="https://github.com/ruuderie" class="button is-info is-inverted">
                <Icon name="uil:github" />
                <span>Github</span>
              </NuxtLink>
        </span>

        </div>
    </div>

      </div>

  </nav>
</template>

<style>
.v-enter-active,
.v-leave-active {
  transition: opacity 0.5s ease;
}

.v-enter-from,
.v-leave-to {
  opacity: 0;
}
</style>
