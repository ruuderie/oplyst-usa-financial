import type { Meta, StoryObj } from '@storybook/vue3';

import TableRow from '../components/ui/table/TableRow.vue';

const meta = {
  title: 'TableRow',
  component: TableRow,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof TableRow>;

export default meta;
type Story = StoryObj<typeof TableRow>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};