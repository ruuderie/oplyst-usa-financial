import type { Meta, StoryObj } from '@storybook/vue3';

import SelectItemText from '../components/ui/select/SelectItemText.vue';

const meta = {
  title: 'SelectItemText',
  component: SelectItemText,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof SelectItemText>;

export default meta;
type Story = StoryObj<typeof SelectItemText>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};