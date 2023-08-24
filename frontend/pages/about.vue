<template>
  <div class="is-responsive is-mobile finance-theme">
    <NuxtLayout>
      <section class="hero finance-hero is-small is-fluid">
        <div class="hero-body is-fluid">
          <div class="my-3 has-text-centered">
            <p class="title">About Oplyst International, LLC</p>
            <p class="subtitle">
              Your Dedicated Partner in Commercial Finance Solutions.
            </p>
          </div>
        </div>
      </section>
      
      <section class="container finance-content is-fluid">
        <div class="my-6 py-4">
          <p>
            As pioneers in the commercial finance sector, we extend our services nationwide, recognizing the pivotal role of capital in spearheading growth and success for businesses. Whether you're at the inception of your small venture or steering a medium-sized enterprise, our commitment is unwavering in ensuring you access the funding you need.
          </p>
          <p>
            Our seasoned team boasts a wealth of knowledge, rooted in decades of collective experience within the commercial finance realm. We're not just financiers; we are specialists in both commercial real estate financing and business growth financing. Our proficiency allows us to discern the distinct challenges that businesses encounter, propelling us to craft strategies that help them not only meet but surpass their aspirations.
          </p>
          <h2>Why Choose Oplyst International, LLC?</h2>
          <ul>
            <li><strong>Inclusivity in Finance:</strong> We champion the belief that every business, irrespective of size, should access the capital they require. Our qualification criteria are designed to be accommodating, enabling a broad spectrum of businesses to benefit from our services.</li>
            <li><strong>Diverse Financial Products:</strong> Our arsenal of financial solutions is varied and comprehensive. Whether it's SBA loans, business acquisition funds, working capital, equity raises for ventures and real estate, or guidance in transitioning to a public entity, we've got you covered.</li>
            <li><strong>Bespoke Solutions:</strong> At Oplyst International, LLC, we don't believe in a one-size-fits-all approach. We delve into understanding your business nuances and aspirations, ensuring the solutions we offer are tailor-made for your unique requirements.</li>
            <li><strong>Unparalleled Client Experience:</strong> Our commitment is not just to finance, but also to ensure your journey with us is smooth and devoid of hassles. Engaging with us assures you of a professional and seamless experience, giving you the tranquillity that your business’s financial realm is adeptly managed.</li>
          </ul>
          <p>
            Reputed for our unwavering dedication and reliability, our name resonates with trust in the industry. Whether you're kickstarting your dream or are an established name, Oplyst International is geared to cater to your financial needs. Reach out to us for a complimentary consultation and let us embark on a successful journey together.
          </p>
          <p class="call-to-action">
                    We're always happy to help! Tell us about your business and its financial needs and we'll help you get the capital you need.
                    <a href="tel:860-333-6524" > (860)-333-6524 </a> or email
                    <a href="mailto:loans@oplystusa.com" > loans@oplystusa.com </a> to chat with our Commercial Finance Consultants.
        </p>
        </div>
        
      </section>
    </NuxtLayout>
  </div>
</template>

<style scoped>
.finance-theme {
    font-family: 'Goldman Sachs', sans-serif;
    background-color: #FFFFFF; /* Updated for better readability */
}

.finance-hero {
    background-color: #0269be; /* Deep blue for the hero section */
    color: #FFFFFF; /* White text on blue background */
}

.finance-content {
    color: #00447A; /* Deep blue for the main content for better readability */
    line-height: 1.6; /* Improved line spacing for better readability */
}

.title, .subtitle {
    margin-bottom: 15px;
}

.container a:hover {
    border-bottom: 2px solid #00447A;
}

.container a:active {
    color: #FFFFFF;
}
</style>
