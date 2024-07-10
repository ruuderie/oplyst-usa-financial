import type { Meta, StoryObj } from '@storybook/vue3';

import Switch from '../components/ui/switch/Switch.vue';

const meta = {
  title: 'Switch',
  component: Switch,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof Switch>;

export default meta;
type Story = StoryObj<typeof Switch>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};