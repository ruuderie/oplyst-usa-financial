import type { Meta, StoryObj } from '@storybook/vue3';

import DropdownMenuContent from '../components/ui/dropdown-menu/DropdownMenuContent.vue';

const meta = {
  title: 'DropdownMenuContent',
  component: DropdownMenuContent,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof DropdownMenuContent>;

export default meta;
type Story = StoryObj<typeof DropdownMenuContent>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};