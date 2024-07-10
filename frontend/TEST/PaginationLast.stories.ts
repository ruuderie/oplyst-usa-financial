import type { Meta, StoryObj } from '@storybook/vue3';

import PaginationLast from '../components/ui/pagination/PaginationLast.vue';

const meta = {
  title: 'PaginationLast',
  component: PaginationLast,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof PaginationLast>;

export default meta;
type Story = StoryObj<typeof PaginationLast>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};