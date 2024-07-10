import type { Meta, StoryObj } from '@storybook/vue3';

import RadioGroupItem from '../components/ui/radio-group/RadioGroupItem.vue';

const meta = {
  title: 'RadioGroupItem',
  component: RadioGroupItem,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof RadioGroupItem>;

export default meta;
type Story = StoryObj<typeof RadioGroupItem>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};