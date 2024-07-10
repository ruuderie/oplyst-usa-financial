import type { Meta, StoryObj } from '@storybook/vue3';

import Table from '../components/ui/table/Table.vue';

const meta = {
  title: 'Table',
  component: Table,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof Table>;

export default meta;
type Story = StoryObj<typeof Table>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};