import type { Meta, StoryObj } from '@storybook/vue3';

import TableCaption from '../components/ui/table/TableCaption.vue';

const meta = {
  title: 'TableCaption',
  component: TableCaption,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof TableCaption>;

export default meta;
type Story = StoryObj<typeof TableCaption>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};