import type { Meta, StoryObj } from '@storybook/vue3';

import TableFooter from '../components/ui/table/TableFooter.vue';

const meta = {
  title: 'TableFooter',
  component: TableFooter,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof TableFooter>;

export default meta;
type Story = StoryObj<typeof TableFooter>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};