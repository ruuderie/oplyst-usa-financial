import type { Meta, StoryObj } from '@storybook/vue3';

import AccordionItem from '../components/ui/accordion/AccordionItem.vue';

const meta = {
  title: 'AccordionItem',
  component: AccordionItem,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof AccordionItem>;

export default meta;
type Story = StoryObj<typeof AccordionItem>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};