import type { Meta, StoryObj } from '@storybook/vue3';

import AccordionContent from '../components/ui/accordion/AccordionContent.vue';

const meta = {
  title: 'AccordionContent',
  component: AccordionContent,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof AccordionContent>;

export default meta;
type Story = StoryObj<typeof AccordionContent>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};