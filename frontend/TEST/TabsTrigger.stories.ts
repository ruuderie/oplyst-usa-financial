import type { Meta, StoryObj } from '@storybook/vue3';

import TabsTrigger from '../components/ui/tabs/TabsTrigger.vue';

const meta = {
  title: 'TabsTrigger',
  component: TabsTrigger,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof TabsTrigger>;

export default meta;
type Story = StoryObj<typeof TabsTrigger>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};