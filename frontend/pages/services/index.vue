<script setup>
import { ref, computed } from "vue";
import LandingPage from "@/components/LandingPage.vue";

const services = ref([
  {
    id: 1,
    name: "Real Estate Loans",
    description:
      "Get the right funding for homes, offices, or construction projects. We're here to back your property dreams.",
    imageSource: "",
  },
  {
    id: 2,
    name: "Equipment Financing",
    description:
      "Don't let the cost of tools slow you down. We'll help you lease the equipment, so you can keep working.",
    imageSource: "",
  },
  {
    id: 3,
    name: "SBA Loans",
    description:
      "Small businesses deserve support. With our government-backed loans, you can fund any aspect of your venture.",
    imageSource: "",
  },
  {
    id: 4,
    name: "Lines of Credit",
    description:
      "Get money when you need it. Only pay for what you use. It's as simple as that.",
    imageSource: "",
  },
  {
    id: 5,
    name: "Merchant Cash Advance",
    description: "Short on cash? We offer quick funds based on your future sales.",
    imageSource: "",
  },
  {
    id: 6,
    name: "Startups",
    description:
      "If you're a VC-Backed start-up we have we've got special finance plans just for you.",
    imageSource: "",
  },
  {
    id: 7,
    name: "Term Loans",
    description:
      "Regular business loans with a fixed end date. Offered by various lenders, including banks and community groups.",
    imageSource: "",
  },
  {
    id: 8,
    name: "Asset-Based Financing",
    description:
      "Use your business assets to get a loan. Whether it's real estate, equipment, or inventory, we've got you covered.",
    imageSource: "",
  },
  {
    id: 9,
    name: "Bridge Loans",
    description:
      "Temporary funding to keep things moving. Useful when you're waiting for other funds to come in.",
    imageSource: "",
  },
]);
</script>

<template>
  <NuxtLayout name="default">
    <section class="section">
      <div class="container">
        <h2 class="has-text-centered is-size-1 statement">How we help</h2>
        <div class="columns is-multiline">
          <div
            class="column is-one-third has-text-centered"
            v-for="service in services"
            :key="service.id"
          >
            <Card
              :id="service.id.toString()"
              :name="service.name"
              :description="service.description"
              :imageSource="service.imageSource"
            />
          </div>
        </div>
      </div>
    </section>
  </NuxtLayout>
</template>
