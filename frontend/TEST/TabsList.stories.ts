import type { Meta, StoryObj } from '@storybook/vue3';

import TabsList from '../components/ui/tabs/TabsList.vue';

const meta = {
  title: 'TabsList',
  component: TabsList,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof TabsList>;

export default meta;
type Story = StoryObj<typeof TabsList>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};