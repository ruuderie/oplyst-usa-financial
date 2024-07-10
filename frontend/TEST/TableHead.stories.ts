import type { Meta, StoryObj } from '@storybook/vue3';

import TableHead from '../components/ui/table/TableHead.vue';

const meta = {
  title: 'TableHead',
  component: TableHead,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof TableHead>;

export default meta;
type Story = StoryObj<typeof TableHead>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};