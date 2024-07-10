import type { Meta, StoryObj } from '@storybook/vue3';

import TableBody from '../components/ui/table/TableBody.vue';

const meta = {
  title: 'TableBody',
  component: TableBody,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof TableBody>;

export default meta;
type Story = StoryObj<typeof TableBody>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};