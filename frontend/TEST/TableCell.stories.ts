import type { Meta, StoryObj } from '@storybook/vue3';

import TableCell from '../components/ui/table/TableCell.vue';

const meta = {
  title: 'TableCell',
  component: TableCell,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof TableCell>;

export default meta;
type Story = StoryObj<typeof TableCell>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};