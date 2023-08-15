<template>
    <div class="page-container container is-fluid">
    <TheNavBar class="page-container__navbar navbar" />
    <div class="page-container container is-flex-wrap-wrap	">

        <main class="page-container__content section">
            <div class="content-wrapper container is-fluid">
                <slot />
            </div>
        </main>
    </div>
    <TheAppFooter class="page-container__footer footer container is-fluid" />
    </div>
</template>


<style scoped>
.page-container {
    background-color: #f4f7fa;
    border: 2px solid #10ff8c;
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