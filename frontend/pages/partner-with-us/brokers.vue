<template>
    <div class="finance-theme is-responsive is-mobile">
        <NuxtLayout>
            <section class="finance-hero is-small is-primary is-fluid">
                <div class="hero-body is-fluid">
                    <div class="my-3 has-text-centered">
                        <p class="title">Broker & Referral Programs</p>
                        <p class="subtitle">
                            At Oplyst International, LLC, the strength of our operations lies in the trusted
                            partnerships we build. We deeply value our ties with brokers, bankers, attorneys, CPAs,
                            consultants, and vendors, understanding that collaboration is the keystone to mutual
                            success.
                        </p>
                    </div>
                </div>
            </section>
            <section class="container finance-content is-fluid">
                <h2 class="title my-4">Why Choose a Partnership with Us?</h2>
                <ul class="benefits-list">
                    <li><strong>Rewarding Commissions & Fees:</strong> Benefit from our competitive commissions and referral fees for every successful transaction you usher in.</li>
                    <li><strong>Your Client Always Remains Yours:</strong> Even if we engage directly with your client for substantial transactions or project financing, we respect the bond you've formed. Should they revisit us in the future, we promptly direct them back to you.</li>
                    <li><strong>Trust and Integrity:</strong> We're not just about business; we're about building lasting, meaningful relationships. With us, your clients and contacts are always safeguarded.</li>
                </ul>
                <p class="call-to-action">
                    Take the leap into a partnership that values your contribution and dedication! Dial 
                    <a target="_blank" href="tel:860-333-6524" > (860)-333-6524 </a> or email
                    <a target="_blank" href="mailto:loans@oplystusa.com" > loans@oplystusa.com </a> to chat with our Commercial Finance Consultants.
                </p>
            </section>
        </NuxtLayout>
    </div>
</template>

<style scoped>
.finance-theme {
    font-family: 'Goldman Sachs', sans-serif; /* Assuming the font is named Goldman Sachs */
    background-color: #FAFAFA; /* Lighter gray background */
}

.finance-hero {
    background-color: #2173b3; 
    color: #EFEFEF; /* Pure white for text */
    padding: 2rem 1rem; /* Padding around content sections */

}

.finance-content {
    background-color: #2173b3; /* Darker navy blue for content */
    color: #EFEFEF; /* Pure white for text */
    padding: 1rem 2rem; /* Padding for the content container */
}

.title, .subtitle {
    margin-bottom: 15px;
    color: #EFEFEF
}

.benefits-list {
    list-style-type: disc;
    padding-left: 2rem;
    margin-bottom: 1.5rem;
}

li {
    margin-bottom: 0.5rem;
}

.call-to-action {
    font-weight: 600; /* Making the call to action a bit bolder */
    margin-top: 1.5rem;
}

.phone-number {
    color: #eff2f9; /* Darker navy blue */
    font-weight: bold; /* Making the phone number stand out */
}

</style>
