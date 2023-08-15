<template>
    <div class="page-container container is-fluid">
        <TheNavBar class="page-container__navbar navbar" />

        <main class="page-container__content section">
            <div class="content-wrapper container is-fluid">
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