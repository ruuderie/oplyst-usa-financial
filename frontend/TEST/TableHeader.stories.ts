import type { Meta, StoryObj } from '@storybook/vue3';

import TableHeader from '../components/ui/table/TableHeader.vue';

const meta = {
  title: 'TableHeader',
  component: TableHeader,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof TableHeader>;

export default meta;
type Story = StoryObj<typeof TableHeader>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};