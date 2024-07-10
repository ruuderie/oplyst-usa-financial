import type { Meta, StoryObj } from '@storybook/vue3';

import TableEmpty from '../components/ui/table/TableEmpty.vue';

const meta = {
  title: 'TableEmpty',
  component: TableEmpty,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof TableEmpty>;

export default meta;
type Story = StoryObj<typeof TableEmpty>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};