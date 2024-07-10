import type { Meta, StoryObj } from '@storybook/vue3';

import AccordionTrigger from '../components/ui/accordion/AccordionTrigger.vue';

const meta = {
  title: 'AccordionTrigger',
  component: AccordionTrigger,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof AccordionTrigger>;

export default meta;
type Story = StoryObj<typeof AccordionTrigger>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};