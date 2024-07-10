import type { Meta, StoryObj } from '@storybook/vue3';

import Select from '../components/ui/select/Select.vue';

const meta = {
  title: 'Select',
  component: Select,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof Select>;

export default meta;
type Story = StoryObj<typeof Select>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};