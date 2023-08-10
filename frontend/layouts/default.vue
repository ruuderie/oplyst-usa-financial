<template>
    <div class="page-container">
        <TheNavBar class="page-container__navbar" />

        <main class="page-container__content">
            <div class="content-wrapper">
                <slot />
            </div>
        </main>

        <TheAppFooter class="page-container__footer" />
    </div>
</template>

<style scoped>
.page-container {
    display: flex;
    flex-direction: column;
    min-height: 100vh;
    background-color: #f4f7fa; /* Light gray for a neutral background */
}

.page-container__navbar {
    flex-shrink: 0;
    background-color: #003366; /* Dark blue for contrast and trustworthiness */
    color: #fff;
}

.page-container__content {
    flex-grow: 1;
    padding: 2rem 1rem;
    background-color: #ffffff; /* White background for the main content area */
}

.content-wrapper {
    max-width: 1200px; /* Define a maximum width for readability */
    margin: 0 auto; /* Centers the content */
    padding: 1rem; /* Additional padding around the content */
}

.page-container__footer {
    flex-shrink: 0;
    background-color: #003366; /* Matching the navbar */
    color: #fff;
}
</style>
