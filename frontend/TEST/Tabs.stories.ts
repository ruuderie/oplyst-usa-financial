import type { Meta, StoryObj } from '@storybook/vue3';

import Tabs from '../components/ui/tabs/Tabs.vue';

const meta = {
  title: 'Tabs',
  component: Tabs,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof Tabs>;

export default meta;
type Story = StoryObj<typeof Tabs>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};