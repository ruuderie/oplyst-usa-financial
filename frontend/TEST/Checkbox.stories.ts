import type { Meta, StoryObj } from '@storybook/vue3';

import Checkbox from '../components/ui/checkbox/Checkbox.vue';

const meta = {
  title: 'Checkbox',
  component: Checkbox,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof Checkbox>;

export default meta;
type Story = StoryObj<typeof Checkbox>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};