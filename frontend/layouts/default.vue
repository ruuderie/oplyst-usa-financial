<template>
  <div class="page-container">
    <TheNavBar class="page-container__navbar navbar container is-fluid" />

    <main class="page-container__content section" style="padding: 0">
      <div class="content-wrapper">
        <slot />
      </div>
    </main>

    <TheAppFooter class="page-container__footer footer container is-fluid" />
  </div>
</template>

<style scoped>
.page-container {
  background-color: #f4f7fa;
}

.page-container__navbar {
  background-color: #003366;
  color: #fff;
}

.content-wrapper {
  /*  margin-top: 2rem; /* Spacing */
  margin-bottom: 2rem; /* Spacing */
}

.page-container__footer {
  background-color: #003366;
  color: #fff;
}
</style>
